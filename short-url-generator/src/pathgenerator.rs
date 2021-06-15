use rand::Rng;
use redis::{Client, RedisResult};
use std::convert::{From, TryFrom};
use std::time::{SystemTime, UNIX_EPOCH};

const TOKENS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const BASE_62: usize = 62;

#[derive(Clone)]
pub struct ShortenerGenerator {
    client: Client,
}

impl ShortenerGenerator {
    pub fn new(client: Client) -> ShortenerGenerator {
        return ShortenerGenerator { client: client };
    }

    pub fn generate_short_url_path(&self, source_url: String) -> String {
        let mut result: Vec<usize> = Vec::new();
        let mut num = self.last_id();
        while num > 0 {
            let r = num % BASE_62;
            result.push(r);
            num = num / BASE_62;
        }
        result.reverse();
        let mut shortened_url_path = String::from("");
        for item in result.iter() {
            shortened_url_path.push(TOKENS.chars().nth(*item).unwrap());
        }
        self.save_shortened_url(&shortened_url_path, &source_url);
        return shortened_url_path;
    }

    fn save_shortened_url(&self, shortened: &String, source_url: &String) {
        let mut conn = self.client.get_connection().unwrap();
        redis::cmd("SET")
            .arg(shortened)
            .arg(source_url)
            .query(&mut conn)
            .unwrap()
    }

    fn last_id(&self) -> usize {
        let mut conn = self.client.get_connection().unwrap();
        let generator: RedisResult<usize> = redis::cmd("INCR").arg("generator").query(&mut conn);
        let id: usize = self.get_random_id();
        match generator {
            Ok(value) => {
                if value == 1 {
                    self.update_generator(id, conn);
                    return id;
                }
                return value;
            }
            Err(_) => self.update_generator(id, conn),
        }
        return id;
    }

    fn update_generator(&self, id: usize, conn: redis::Connection) {
        let mut c = conn;
        redis::cmd("SET")
            .arg("generator")
            .arg(id)
            .query(&mut c)
            .unwrap()
    }

    fn get_random_id(&self) -> usize {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut rng = rand::thread_rng();
        let id: u128 = time * rng.gen_range(1..999);
        return usize::try_from(id).unwrap();
    }
}
