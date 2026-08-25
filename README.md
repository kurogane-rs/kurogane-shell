# kurogane-shell

The Rust shell [Kurogane](https://github.com/0x48piraj/kurogane) generates
when integrating with an existing frontend project.

Your existing files are never touched. To create a fresh project instead,
use templates via `kurogane new`.

## Placeholders

| Placeholder | Meaning                              | Default |
|-------------|--------------------------------------|---------|
| `frontend`  | Frontend assets directory            | `dist`  |
| `dev_url`   | Dev server URL used by `kurogane dev`| *(empty)* |

## Usage

```sh
cd my-app
kurogane init
kurogane dev
```
