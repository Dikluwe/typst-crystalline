# P1329 — RED confirmado antes de C

**Veredito: RED real e suficiente para liberar a implementação no owner calc.**
Não há falha de compilação/harness nem causa nova fora do escopo nas falhas
observadas. O veredito não antecipa GREEN dos casos posteriores nas tabelas.

Revisor `/root/p1329_review`, A/B sem atestação técnica de isolamento e sem
refinement seal. Somente leitura dos artefatos testados.

Recibo `p1329-unit-red.json`, SHA-256
`a675be4c1a0811e80735cfa6554e5f94ea86a0bcf1dc62d353794e3620c632ad`;
manifesto R2 `c7f5963d2735226ae3ddf653deb13856e187f723312fcfd7521c42ae5011fbe1`.
Execução `2026-09-09T12:35:52.022315+00:00` até
`2026-09-09T12:37:43.196470+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
com diff/stat/inventários integrais no recibo.

```text
CARGO_TARGET_DIR=/tmp/p1329-target.bg3p5A
cargo test --release --locked -p typst-core compiler::stdlib::calc::p132 -- --nocapture
```

O build terminou e executou o harness `typst_core-34577d6f7c6d8e68`.
Resultado: 15 testes executados, 8 verdes, 7 falhas, zero ignorados, exit 101.
Ambos os módulos P1328/P1329 foram selecionados.

As falhas de espécie, não finitos, sucessor P1328 e unidades públicas encontram
a rejeição histórica `calc.abs() requer Int ou Float, recebeu length` ao
esperar sucesso dimensional. As falhas de comprimento misto encontram essa
mesma mensagem ao esperar `cannot take absolute value of this length`.
O teste de warning chega à exigência de sucesso em `calc.abs(-2em)` depois
de aprovar o warning; falha no resultado ainda rejeitado, consistente com a
mesma lacuna dimensional. Os guards nativos e os controles de conteúdo/math
P1328 permanecem verdes.

Como as tabelas interrompem no primeiro caso falho, este RED demonstra o
defeito inicial de Length; não afirma ter executado todos os casos Angle,
Ratio, Fraction, sinais ou perfis posteriores. GREEN dos mesmos testes terá
de atravessar essas tabelas integralmente.

Comparações independentes confirmaram inventários produtivos idênticos antes
e depois do RED, iguais ao estado posterior da integração congelada. O owner
atual ainda reconstrói exatamente o baseline com os snippets congelados e
mudança exclusiva da linha de prompt-hash: nenhuma C funcional antecedeu RED.
