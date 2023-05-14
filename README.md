# Connect 2

## Run

```bash
cargo run
```

## Usage

1. Go to `http://localhost:3000/groups/{email}` to see the groups for a given member.
2. Go to `http://localhost:3000/groups/{email}/slugs` to see the slugs of the groups for a given member.

## Development Token

Download your service account key and place it in `key.json`.

```bash
pip install -r requirements.txt
echo "GOOGLE_CLOUD_ACCESS_TOKEN=$(python google.py)" >> .env
```
