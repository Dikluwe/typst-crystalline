# P1293/C — recibo de reabertura e fechamento L0 do owner gap `export/mod`

**Estado:** owner gap fechado no L0; aguardando resselo mecânico de lineage,
gate discriminatório substituto e selo separado  
**Papel:** `autor_contrato_p1293` — autoria de obrigação/L0, sem autoridade
sobre produto, oráculo, testes ou veredito  
**Instante da medição:** `2026-09-02T14:18:28-03:00`  
**HEAD:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507`  
**Working tree:** não commitada; `git diff HEAD --stat` enumerou 61 ficheiros
alterados no instante da medição

## Entradas congeladas

- manifesto anterior ao fechamento:
  `00_nucleo/diagnosticos/p1293-manifest.json`, SHA-256
  `8c3ec13e0cf82f43f5c64bd577a20a1001422f052ed7cafd1502d68122fd6115`;
- selo C consumido pelo STOP:
  `00_nucleo/diagnosticos/p1293-contract-seal.json`, SHA-256
  `b6466b5acd4b91fec5b876e0a141fb79d557d8d472d8adf0ba869be7a9430258`;
- gate discriminatório fresco C: SHA-256
  `3eb10ac5f92c49a1ec5c94e4c6f0c4129035a9f1da86f5c0b7801508de9c3cb2`;
- contrato canônico: SHA-256
  `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`;
- oráculo protegido: apenas o pin já publicado foi usado, SHA-256
  `2947f78ea62b518edf6753253dd5b1d8dba46e1caa60cb033566c70a6e491568`;
  o autor não leu nem alterou o seu corpo.

## Medição anterior à decisão

1. `03_infra/src/export/mod.rs:24` declara `mod html;`, mantendo o submódulo
   privado.
2. `03_infra/src/export/mod.rs:534` reexporta somente `export_html`.
3. O diff parcial recebido faz `03_infra/src/pipeline.rs:168,181,189` nomear
   `crate::export::html::HtmlSerializationMode` e `:213` nomear
   `crate::export::html::export_html_with_serialization`; um módulo irmão não
   deve atravessar essa fronteira privada.
4. `00_nucleo/prompts/infra/export/html.md` já é dono da definição e semântica
   de `HtmlSerializationMode`, `export_html` e
   `export_html_with_serialization`; `infra/pipeline.md` já é dono do
   transporte. O L0 `infra/export/mod.md`, que enumera a fachada pública do
   seu consumer 1:1, não mencionava o enum nem a nova entry point.
5. Hashes medidos antes da autoria:
   - `infra/export/mod.md`:
     `bdf4ec679daede866f8f1f303cd17389f6e5f3c2cc0d06849132903870eaa6ce`;
   - `export/mod.rs`:
     `56d2484579e26715fa870b66cdbfb6aaca8aa88a674d1ab9b9efefda2490632c`;
   - `infra/export/html.md`:
     `505895669df4a1d7ce39f4c3236d6ac4e39fc9c61ce14d8cf578b0d394a2d237`;
   - `infra/pipeline.md`:
     `ce6da4f623a0270869606bdf42d07dae0283f3f9e5a593258609b219424e63a8`;
   - candidato parcial `export/html.rs`:
     `c081108b8518556c7fa920f9aabdc9e2f40facccfc90c7ed661ffc780cdaca9d`;
   - candidato parcial `pipeline.rs`:
     `88dd7075b58e0f013c0942a72e52a7de7b104ebc087812a8a96f56b82ab7d167`.

## Decisão e classificação

O owner `00_nucleo/prompts/infra/export/mod.md` passa a exigir que
`03_infra/src/export/mod.rs` mantenha `html` privado e reexporte, sem wrapper
ou lógica, `export_html`, `export_html_with_serialization` e
`HtmlSerializationMode`. Callers usam a fachada `crate::export::*`.

O owner `export/html.md` conserva toda a autoria semântica: enum, funções,
default cristalino, escaping e morfologia. `export/mod.md` possui somente a
visibilidade/caminho público. PDF, PNG e SVG permanecem byte-conceitualmente
inalterados; é proibido duplicar o enum, criar forwarding, mover lógica ou
alterar feature, target, default, fase, entidade ou outro exporter.

- **ADR-0107:** reexport versus módulo privado é mecânica de composição; as
  semânticas HTML continuam no owner `export/html.md`.
- **ADR-0108:** a decisão sucede às medições file:line e hashes acima; seria
  refutada se Rust permitisse ao sibling nomear o módulo privado ou se a
  fachada já expusesse ambos os símbolos, o que a fonte medida nega.
- **ADR-0127:** fechamento documental/operacional da API L3 já confirmada em
  `2026-09-02T13:32:08-03:00`; nenhum contrato público adicional, default ou
  fase nova é introduzido, portanto não há novo gate humano substantivo.
- **ADR-0129:** ownership permanece estritamente
  `infra/export/mod.md` ↔ `export/mod.rs`; `export/html.md` não passa a possuir
  um segundo consumer e nenhum núcleo novo é necessário.

## Resultado da autoria

- L0 alterado: `00_nucleo/prompts/infra/export/mod.md`;
- SHA-256 L0 antes:
  `bdf4ec679daede866f8f1f303cd17389f6e5f3c2cc0d06849132903870eaa6ce`;
- SHA-256 L0 depois:
  `3e0e025b2c7429fd47cb9d77b06b0cd2922ee8bd43252ae211b79ccf3cb8a6e5`;
- consumer `03_infra/src/export/mod.rs` preservado em
  `56d2484579e26715fa870b66cdbfb6aaca8aa88a674d1ab9b9efefda2490632c`;
- produto, headers, testes, oráculo, contrato, manifesto e selo não foram
  escritos por esta autoria.

O selo `b6466b5a…` fica causalmente consumido/inválido pela mudança do L0, mas
o seu arquivo permanece byte-idêntico. O coordenador deve primeiro ressellar
somente o header de `export/mod.rs`, atualizar o manifesto, revalidar as
entradas protegidas e obter gate substituto antes de um replacement seal que
possa acrescentar `03_infra/src/export/mod.rs` à allowlist.

## Validações e dry-run esperado

- V15: PASS, zero violações;
- V26: PASS, zero violações;
- `crystalline-lint --fix-hashes --dry-run .`: listou exatamente
  `03_infra/src/export/mod.rs`, `old=a181f89d`, `hash-a=050dfb36`,
  `hash-b=4ee50bcb`;
- `git diff --check`: PASS;
- `crystalline-lint --fix-hashes .` não foi executado.
