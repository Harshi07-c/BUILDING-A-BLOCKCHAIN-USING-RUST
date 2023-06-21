mod models;
fn main() {
   let difficulty = 1;
   let nonce = String::from("ABC");
   let mut blockchain = models::blockchain::Blockchain::new(difficulty);
   models::blockchain::Blockchain::add_block(&mut blockchain,nonce);
}