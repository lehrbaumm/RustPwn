pub mod tubes;
pub mod util;
pub mod binary;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn spawn_process_kill() {
        use crate::tubes::process::ProcessBuilder;
        use crate::tubes::Tube;

        let p_result = ProcessBuilder::new("cat");

        assert!(p_result.is_ok(), "ProcessBuilder couldn't be created");

        let mut p = p_result.unwrap().build().unwrap();

        
        assert!(p.close().await.is_ok(), "Process didn't exit!");
    }

    
    #[tokio::test]
    async fn test_process_readline() {
        use crate::tubes::process::ProcessBuilder;
        use crate::tubes::Tube;
        use bytes::Bytes;

        let p_result = ProcessBuilder::new("echo");

        assert!(p_result.is_ok(), "ProcessBuilder couldn't be created");

        let mut p = p_result.unwrap().arg("Test").build().unwrap();

        let line_result = p.recvline(false, None).await;

        assert!(line_result.is_ok(), "Couldn't read a line!");

        let line = line_result.unwrap();

        assert_eq!(line, Bytes::from("Test"));
    }
    
}
