use redis::{Client, RedisResult};
use std::convert::{From, TryFrom};
use std::time::{SystemTime, UNIX_EPOCH};

const TOKENS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const BASE_62: usize = 62;

#[derive(Clone)]
pub struct ShortenerGenerator<T>
where
    T: UrlPathService + Clone,
{
    path_writer: T,
}

#[derive(Clone)]
pub struct UrlPathWriter {
    client: Client,
}

pub trait UrlPathService {
    fn save_shortened_url(&self, shortened: &String, source_url: &String);
    fn get_last_shortened_id(&self) -> usize;
}

impl UrlPathWriter {
    pub fn new(client: Client) -> UrlPathWriter {
        return UrlPathWriter { client: client };
    }
    fn get_random_id(&self) -> usize {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        return usize::try_from(time).unwrap();
    }

    fn update_generator(&self, id: usize, conn: redis::Connection) {
        let mut c = conn;
        redis::cmd("SET")
            .arg("generator")
            .arg(id)
            .query(&mut c)
            .unwrap()
    }
}

impl UrlPathService for UrlPathWriter {
    fn save_shortened_url(
        &self,
        shortened: &std::string::String,
        source_url: &std::string::String,
    ) {
        let mut conn = self.client.get_connection().unwrap();
        redis::cmd("SET")
            .arg(shortened)
            .arg(source_url)
            .query(&mut conn)
            .unwrap()
    }

    fn get_last_shortened_id(&self) -> usize {
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
}

impl<T> ShortenerGenerator<T>
where
    T: UrlPathService + Clone,
{
    pub fn new(path_writer: T) -> ShortenerGenerator<T> {
        return ShortenerGenerator {
            path_writer: path_writer,
        };
    }

    pub fn generate_short_url_path(&self, source_url: String) -> String {
        let mut result: Vec<usize> = Vec::new();
        let mut num = self.path_writer.get_last_shortened_id();
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
        self.path_writer
            .save_shortened_url(&shortened_url_path, &source_url);
        return shortened_url_path;
    }
}
