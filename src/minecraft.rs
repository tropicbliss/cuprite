use anyhow::Result;
use rcon::Connection;

pub struct Server {
    client: Connection,
}

impl Server {
    pub async fn new(port: u16, password: String) -> Result<Self> {
        let addr = format!("localhost:{}", port);
        let client = Connection::builder()
            .enable_minecraft_quirks(true)
            .connect(addr, &password)
            .await?;
        Ok(Self { client })
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.client.cmd("say Starting backup...").await?;
        self.client.cmd("save-off").await?;
        self.client.cmd("save-all").await?;
        Ok(())
    }

    pub async fn disconnect(&mut self, is_success: bool) -> Result<()> {
        self.client.cmd("save-on").await?;
        if is_success {
            self.client.cmd("say Backup complete!").await?;
        } else {
            self.client
                .cmd("say Backup is not saved! Please notify an administrator")
                .await?;
        }
        Ok(())
    }
}
