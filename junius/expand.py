import subprocess
import sys

# folder and package name.
packages = [('core', 'substrate-client'),
			('core', 'substrate-consensus-authorities'),
			('core', 'substrate-consensus-common'),
			('core', 'substrate-consensus-aura'),
			('core', 'substrate-consensus-babe'),
			('core', 'substrate-executor'),
			('core', 'substrate-finality-grandpa'),
			('core', 'substrate-primitives'),
			('core', 'substrate-service'),
			('core', 'sr-io'),
			('core', 'sr-primitives'),
			('core', 'sr-sandbox'),
			('core', 'sr-std'),
			('core', 'sr-version'),
			('node', 'node-primitives'),
			('node', 'node-runtime'),
			('node', 'node-cli'),
			('srml', 'srml-assets'),
			('srml', 'srml-aura'),
			('srml', 'srml-babe'),
			('srml', 'srml-balances'),
			('srml', 'srml-consensus'),
			('srml', 'srml-contract'),
			('srml', 'srml-council'),
			('srml', 'srml-democracy'),
			('srml', 'srml-example'),
			('srml', 'srml-executive'),
			('srml', 'srml-finality-tracker'),
			('srml', 'srml-grandpa'),
			('srml', 'srml-indices'),
			('srml', 'srml-metadata'),
			('srml', 'srml-session'),
			('srml', 'srml-staking'),
			('srml', 'srml-sudo'),
			('srml', 'srml-support'),
			('srml', 'srml-system'),
			('srml', 'srml-timestamp'),
			('srml', 'srml-treasury')
			]

command_prefix = 'cargo +nightly rustc --profile=check --package '
command_postfix = ' --lib -- -Zunstable-options --pretty=expanded > junius/expands/'

# user_command = command_prefix + 'node-template-runtime' + command_postfix + 'node-template/node-template-runtime.rs'
# shell_output = subprocess.Popen(user_command, shell=True, stdout=subprocess.PIPE).stdout.read()


# loop node
for item in packages:
	command = command_prefix + item[1] + command_postfix + item[0] + '/' + item[1] + '.expand.rs'
	print(command)
	shell_output = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE).stdout.read()
