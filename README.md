# token-gen
A simple JSON Web Token (JWT) generator.

## Usage
Create a spec file in TOML like below:

```toml
[generic]
project = "<project_name>"
algorithm = "HS256"
secret = "<secret>"

[payload]
uid = "<uid>"
scopes = []  # List of string defined scopes.
```

Execute with command, like below:
```bash
# You can run with Cargo directly:
$ cargo run <spec_file_path>

# or you can
$ token-gen <spec_file_path>
```

## Known issues:
- Does not support any other algorithms except `HS256`.
- Spec file does not support custom fields.

## License
BSD 3-Clause License
