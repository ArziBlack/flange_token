## first tiral (solana and rust compiler issues)

[dependencies]
solana-program = "2.1.0"
spl-token = "3.5.0"  
borsh = "0.9.1"

## second trial (stable config by solana devs solutions)

[dependencies]
solana-program = "=1.17.0"
spl-token = { version="3.5.0", features = [ "no-entrypoint" ] }  
borsh = "0.10.3"
ahash = "=0.8.6"
