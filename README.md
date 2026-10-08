# kurogane-shell

The Rust shell [Kurogane](https://github.com/0x48piraj/kurogane) generates
when integrating with an existing frontend project.

Your existing files are never touched. To create a fresh project instead,
use templates via `kurogane new`.

## Placeholders

| Placeholder     | Meaning                                         | Default   |
|-----------------|-------------------------------------------------|-----------|
| `frontend_dist` | Frontend build output directory                 | `dist`    |
| `dev_url`       | Dev server URL for debug builds; empty loads the build output | *(empty)* |

## Usage

```sh
cd my-app
kurogane init
kurogane dev
```
