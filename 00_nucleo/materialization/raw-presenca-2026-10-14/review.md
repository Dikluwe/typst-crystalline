# Proposta: presença lexical em Raw

**Aguardando confirmação. Nenhum campo público foi implementado.**

Adicionar `pub set_fields: u8` a `RawElem`, com constantes `RAW_SET_BLOCK = 1`
e `RAW_SET_LANG = 2`, para distinguir omissão de `block: false` e `lang: none`.
Hoje essa informação se perde antes dos getters e serializers; ela também
muda a igualdade da linguagem. O restante dos campos e a assinatura de
`Content::raw` permanecem. Construtores Rust externos por struct literal
precisarão informar o campo novo.

O candidato L0 completo está em
[candidates/00_nucleo/prompts/entities/elements/raw.md](candidates/00_nucleo/prompts/entities/elements/raw.md).
O núcleo candidato está em
[candidates/00_nucleo/prompts/_nuclei/raw-field-presence.toml](candidates/00_nucleo/prompts/_nuclei/raw-field-presence.toml).
As responsabilidades dos consumers e os hashes dos arquivos atuais estão no
[manifesto](manifest.json).

SHA-256 do manifesto: `b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45`.

Após confirmação, os candidatos serão instalados nos caminhos canônicos de
L0, os prompts dependentes serão atualizados, e os testes RED precederão o
código. O gate cobre presença, campos, igualdade, observação e serialização;
não muda fase, render/defaults, RawLine, ou casts/aceitações legadas de raw().
As divergências restantes do constructor continuam documentadas para decisão
própria. Não se usará metadata em Styled, registry global ou Location falsa
para evitar a alteração tipada.

A confirmação é exigida pelo repositório em
`00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md`, §2: adicionar campo
público e quebrar construção externa requerem parar antes de L1. Os reparos
independentes em Empty/Source continuam sendo validados em fluxo contínuo.
