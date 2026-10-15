use std::collections::BTreeSet;

use acvc_domain::{Error, Hash, Result, Variant, decode_tree, validate_campaign_name};

use crate::App;

pub struct ObjectBytes {
    pub hash: Hash,
    pub kind: u8,
    pub bytes: Vec<u8>,
}

pub struct Offer {
    pub variant: Hash,
    pub canonical: Vec<u8>,
    pub objects: Vec<ObjectBytes>,
}

pub struct AcceptReport {
    pub variant: Hash,
    pub graph: &'static str,
}

impl App {
    pub fn offer_variant(&self, id: &Hash) -> Result<Offer> {
        let stored = self.ports.variants.get_variant(id)?;
        let canonical = stored.variant.canonical()?;
        if stored.variant.id()? != *id {
            return Err(
                Error::new("stored variant does not match its id").ctx("variant", id.to_hex())
            );
        }
        Ok(Offer {
            variant: *id,
            objects: self.closure(stored.variant.tree)?,
            canonical,
        })
    }

    pub fn objects_present(&self, hashes: &[Hash]) -> Result<Vec<Hash>> {
        let mut present = Vec::new();
        for hash in hashes {
            if self.ports.objects.has_object(hash)? {
                present.push(*hash);
            }
        }
        Ok(present)
    }

    pub fn read_objects(&self, hashes: &[Hash]) -> Result<Vec<ObjectBytes>> {
        let mut objects = Vec::with_capacity(hashes.len());
        for hash in hashes {
            let (kind, bytes) = self.ports.objects.read_object(hash)?;
            objects.push(ObjectBytes {
                hash: *hash,
                kind,
                bytes,
            });
        }
        Ok(objects)
    }

    pub fn accept_variant(
        &self,
        campaign: &str,
        canonical: &[u8],
        objects: &[ObjectBytes],
    ) -> Result<AcceptReport> {
        validate_campaign_name(campaign)?;
        let variant = Variant::decode(canonical)?;
        let id = variant.id()?;
        for object in objects {
            let stored = self.ports.objects.put_object(object.kind, &object.bytes)?;
            if stored != object.hash {
                return Err(Error::new("object hash mismatch").ctx("hash", object.hash.to_hex()));
            }
        }
        self.require_closure(variant.tree)?;
        let graph = match self.ports.variants.get_variant(&id) {
            Ok(existing) => {
                if existing.variant.canonical()? != canonical {
                    return Err(Error::new("refusing to rewrite stored variant")
                        .ctx("variant", id.to_hex()));
                }
                "unchanged"
            }
            Err(err) if err.message == "variant not found" => {
                self.ports.variants.insert_variant(
                    campaign,
                    &variant,
                    self.ports.clock.now_millis(),
                )?;
                "inserted"
            }
            Err(err) => return Err(err),
        };
        Ok(AcceptReport { variant: id, graph })
    }

    fn closure(&self, root: Hash) -> Result<Vec<ObjectBytes>> {
        let mut objects = Vec::new();
        let mut seen = BTreeSet::new();
        self.walk(root, &mut objects, &mut seen)?;
        Ok(objects)
    }

    fn walk(
        &self,
        hash: Hash,
        objects: &mut Vec<ObjectBytes>,
        seen: &mut BTreeSet<Hash>,
    ) -> Result<()> {
        if !seen.insert(hash) {
            return Ok(());
        }
        let (kind, bytes) = self.ports.objects.read_object(&hash)?;
        if kind == 1 {
            let entries = decode_tree(&bytes)?;
            objects.push(ObjectBytes { hash, kind, bytes });
            for entry in entries {
                self.walk(entry.hash, objects, seen)?;
            }
            return Ok(());
        }
        if kind != 0 {
            return Err(Error::new("unknown object kind").ctx("kind", kind));
        }
        objects.push(ObjectBytes { hash, kind, bytes });
        Ok(())
    }

    fn require_closure(&self, root: Hash) -> Result<()> {
        let mut pending = vec![root];
        let mut seen = BTreeSet::new();
        while let Some(hash) = pending.pop() {
            if !seen.insert(hash) {
                continue;
            }
            if !self.ports.objects.has_object(&hash)? {
                return Err(Error::new("missing object").ctx("hash", hash.to_hex()));
            }
            let (kind, bytes) = self.ports.objects.read_object(&hash)?;
            if kind == 1 {
                for entry in decode_tree(&bytes)? {
                    pending.push(entry.hash);
                }
            }
        }
        Ok(())
    }
}
