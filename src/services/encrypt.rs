use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub struct Encrypt<'a>(pub &'a str);

impl<'a> Encrypt<'a> {
    pub fn encrypt(&self, value: &str) -> String {
        let mc = new_magic_crypt!(self.0, 256);
        mc.encrypt_str_to_base64(value)
    }
    
    pub fn decrypt(&self, value: &str) -> String {
        let mc = new_magic_crypt!(self.0, 256);
        mc.decrypt_base64_to_string(value).unwrap()
    }
}
