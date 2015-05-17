#[derive(Debug)]
pub struct Client<'client> {
    access_token: &'client str,
    environment: &'client str,
    version: &'client str,
    app_name: &'client str
}

impl<'client> Client<'client> {
    pub fn new(access_token: &'client str, environment: &'client str, app_name: &'client str, version: &'client str) -> Client<'client> {
        Client {
            access_token: access_token,
            environment: environment,
            app_name: app_name,
            version: version
        }
    }

    pub fn environment(&self) -> &str {
        &self.environment
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }
}

impl<'client> PartialEq for Client<'client> {
    fn eq(&self, other: &Client) -> bool {
        let other_token = other.access_token();
        let other_env = other.environment();
        let other_ver = other.version();
        let other_app = other.app_name();

        &self.access_token[..] == &other_token[..] &&
        &self.environment[..] == &other_env[..] &&
        &self.app_name[..] == &other_app[..] &&
        &self.version[..] == &other_ver[..]
    }
}

impl<'client> Eq for Client<'client> {}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn test_constructor() {
        let access_token = "abc";
        let environment = "test";
        let app_name = "my_app";
        let version = "0.1.0";

        let client = Client::new(&access_token[..], &environment[..], &app_name[..], &version[..]);

        let expected_client = Client {
            access_token: access_token,
            environment: environment,
            app_name: app_name,
            version: version
        };

        assert_eq!(expected_client, client);
    }
}
