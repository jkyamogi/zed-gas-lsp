# Zed G.A.S. LSP
### Zed extension for Google Apps Script Syntax Highlighting and Intellisense

## Set Up
#### At project level
pnpm init -y
pnpm add @types/google-apps-script typescript

#### Create tsconfig.json in project root
```json
{
  "compilerOptions": {
    "allowJs": true,
    "checkJs": true,
    "noEmit": true,
    "target": "ES2020",
    "lib": ["ES2020"],
    "types": ["google-apps-script"]
  },
  "include": ["**/*.gs", "**/*.js", "**/*.ts"]
}
```
