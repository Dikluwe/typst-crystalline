# Prompt L0 — comando `typst init`
Hash do Código: e636235b

**Camadas:** L2/L3/L4  
**Ficheiros alvo:** `02_shell/src/cli.rs`, `03_infra/src/project_init.rs`,
`04_wiring/src/main.rs`  
**Estado:** INIT-1 e INIT-2 implementados

## Medição anterior à decisão

O vanilla recebe `TEMPLATE [DIR]`, aceita spec com ou sem versão e usa o nome
do package como diretório default (`args.rs:135-150`; `init.rs:17-47`). Exige
`[template]` no `typst.toml`, recusa destino existente e copia somente o
conteúdo da pasta do template (`init.rs:38-47,69-92`). O cristalino já resolve
packages locais/remotos em `SystemWorld`, mas não possui parser/materializador
de template nem comando `init`.

**Classificação:** semântica pública e I/O de produto. Inferência: reutilizar
apenas o entrypoint de compilação não basta; um manifesto sem `[template]`
refuta essa abordagem.

## Contrato público inicial

```text
typst init TEMPLATE [DIR]
```

1. Aceita `@namespace/name:version` e `@namespace/name`; sem versão aplica a
   resolução definida em `infra/package_version_resolution.md`.
2. Resolve primeiro cache local e só usa downloader já configurado quando
   necessário.
3. Valida identidade/versionamento do manifesto e presença de `[template]`
   com `path` e `entrypoint` relativos.
4. Destino omitido usa o nome do package.
5. Destino existente falha sem escrever nada.
6. Path do template não pode escapar da raiz do package.
7. Materialização usa staging no mesmo filesystem e rename; falha não deixa
   projeto parcial.
8. Sucesso imprime diretório e entrypoint criado.

## Entregas concluídas

`P1137-C001-INIT-1` implementa versão explícita e
`P1137-C001-INIT-2` implementa versão omitida consumindo
`infra/package_version_resolution.md`. Ambas estão materializadas.

## Topologia e segurança

- L2 define `InitIntent`; L3 lê TOML/copia; L4 compõe.
- Não sobrescrever destino, não seguir paths `..` para fora do package e não
  copiar symlink cujo alvo escape a raiz.
- Nenhuma lógica de filesystem entra em L1.

## Testes e aceitação

Fixture local template: sucesso; destino default; destino existente; package
sem template; path escapando; falha no meio sem árvore parcial; e, em INIT-2,
seleção da versão local mais recente. Mensagens públicas são comparadas com o
vanilla quando equivalentes.

## Gate

Novo comando e escrita material no filesystem: confirmação obrigatória.
