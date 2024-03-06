use std::process::{Command, Stdio};

// returns previous state: "running\n" or something else
pub fn aws_start_cmd() -> String {
    let instance_id = std::env::var("INSTANCE_ID").expect("missing INSTANCE_ID");
    let aws_cmd = Command::new("aws")
        .arg("ec2")
        .arg("start-instances")
        .arg("--no-cli-pager")
        .arg("--instance-ids")
        .arg(&instance_id)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let jq_cmd = Command::new("jq")
        .arg("-r")
        .arg(".StartingInstances[].PreviousState.Name")
        .stdin(Stdio::from(aws_cmd.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output();
    return String::from_utf8_lossy(&jq_cmd.unwrap().stdout).to_string();
    // return String::from_utf8_lossy(&aws_cmd.unwrap().stdout).to_string();
    // return aws_cmd.unwrap().status.success();
}

pub fn aws_state_cmd() -> String {
    let instance_id = std::env::var("INSTANCE_ID").expect("missing INSTANCE_ID");
    let aws_cmd = Command::new("aws")
        .arg("ec2")
        .arg("describe-instances")
        .arg("--no-cli-pager")
        .arg("--instance-ids")
        .arg(&instance_id)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let jq_cmd = Command::new("jq")
        .arg("-r")
        .arg(".Reservations[].Instances[].State.Name")
        .stdin(Stdio::from(aws_cmd.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output();
    return String::from_utf8_lossy(&jq_cmd.unwrap().stdout).to_string();
}

pub fn aws_ip_cmd() -> String {
    let instance_id = std::env::var("INSTANCE_ID").expect("missing INSTANCE_ID");
    let aws_cmd = Command::new("aws")
        .arg("ec2")
        .arg("describe-instances")
        .arg("--no-cli-pager")
        .arg("--instance-ids")
        .arg(&instance_id)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let jq_cmd = Command::new("jq")
        .arg("-r")
        .arg(".Reservations[].Instances[].PublicIpAddress")
        .stdin(Stdio::from(aws_cmd.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output();
    return String::from_utf8_lossy(&jq_cmd.unwrap().stdout).to_string();
}

#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};

    // ignore tests to avoid build failure
    // uncomment for debugging
    #[ignore]
    #[test]
    fn aws_cmd() {
        let instance_id = std::env::var("INSTANCE_ID").expect("missing INSTANCE_ID");
        let aws_cmd = Command::new("aws")
            .arg("ec2")
            .arg("describe-instances")
            .arg("--instance-ids")
            .arg(instance_id)
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();
        println!("{:#?}", aws_cmd);
    }
    #[ignore]
    #[test]
    fn aws_and_jq_cmd() {
        let instance_id = std::env::var("INSTANCE_ID").expect("missing INSTANCE_ID");
        println!("{}", instance_id);
        let aws_cmd = Command::new("aws")
            .arg("ec2")
            .arg("describe-instances")
            .arg("--no-cli-pager")
            .arg("--instance-ids")
            .arg(&instance_id)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let jq_cmd = Command::new("jq")
            .arg("-r")
            .arg(".Reservations[].Instances[].State.Name")
            .stdin(Stdio::from(aws_cmd.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output();
        println!("{}", String::from_utf8_lossy(&jq_cmd.unwrap().stdout));
    }
}
