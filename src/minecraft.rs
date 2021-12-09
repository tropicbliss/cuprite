use crate::rcon::Client;
use anyhow::Result;

pub struct Server {
    client: Client,
    password: String,
}

impl Server {
    pub fn new(port: u16, password: String) -> Result<Self> {
        let addr = format!("0.0.0.0:{}", port);
        let client = Client::new(&addr)?;
        Ok(Self { client, password })
    }

    pub fn connect(&mut self) -> Result<()> {
        self.client.authenticate(&self.password)?;
        self.client
            .send_command("say [\u{a7}WARN\u{a7}r] Starting backup...")?;
        self.client.send_command("save-off")?;
        self.client.send_command("save-all")?;
        self.client.close()?;
        Ok(())
    }

    pub fn disconnect(&mut self, is_success: bool) -> Result<()> {
        self.client.send_command("save-on")?;
        if is_success {
            self.client
                .send_command("say [\u{a7}INFO\u{a7}r] Backup complete!")?;
        } else {
            self.client.send_command(
                "say [\u{a7}ERROR\u{a7}r] Backup is not saved! Please notify an administrator",
            )?;
        }
        Ok(())
    }
}
