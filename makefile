# to test from CLI:

state:
		aws ec2 describe-instances --instance-ids ${INSTANCE_ID} | jq -r '.Reservations[].Instances[].State.Name'

ip:
	aws ec2 describe-instances --no-cli-pager --instance-ids ${INSTANCE_ID} | jq -r '.Reservations[].Instances[].PublicIpAddress'
	
start:
	aws ec2 start-instances --no-cli-pager --instance-ids ${INSTANCE_ID} | jq -r '.StartingInstances[].PreviousState.Name'
