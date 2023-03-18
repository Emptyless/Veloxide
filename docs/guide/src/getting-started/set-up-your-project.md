# Set up your project

Install the Veloxide CLI:

```sh
> cargo install veloxide
```

Create your own app:

```zsh
> veloxide init my-app

# Go to the created folder
> cd my-app

# Install the required tools for development
> just install-required

# Set the environment to use the Postgres config, start the supporting containers, and then run the app
> just dev-postgres

# Once done, open `my-app/` in your IDE

# Happy Coding!
```
