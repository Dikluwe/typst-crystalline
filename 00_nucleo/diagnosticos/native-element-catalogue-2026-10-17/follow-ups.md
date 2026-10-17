# Pendências depois do lote de identidade nativa

A matriz mede bindings efetivos do registry em oito perfis; não cobre todos os campos associados de Type, closures de usuário nem todas as capacidades de layout. `before.json.gz`, `before-measurement.json.gz`, `batch-schema.json` e a validação final fixam a proveniência.

## math

- `std.math.accent`
- `std.math.attach`
- `std.math.binom`
- `std.math.cancel`
- `std.math.cases`
- `std.math.class`
- `std.math.frac`
- `std.math.limits`
- `std.math.lr`
- `std.math.mat`
- `std.math.mid`
- `std.math.overbrace`
- `std.math.overbracket`
- `std.math.overline`
- `std.math.overparen`
- `std.math.overshell`
- `std.math.primes`
- `std.math.root`
- `std.math.scripts`
- `std.math.stretch`
- `std.math.underbrace`
- `std.math.underbracket`
- `std.math.underline`
- `std.math.underparen`
- `std.math.undershell`
- `std.math.vec`

## formats/html/pdf

- `std.format.bundle`
- `std.format.html`
- `std.format.html.elem`
- `std.format.html.frame`
- `std.format.pdf`
- `std.format.pdf.artifact`
- `std.format.pdf.attach`
- `std.format.png`
- `std.format.svg`
- `std.html`
- `std.html.elem`
- `std.html.frame`
- `std.pdf`
- `std.pdf.artifact`
- `std.pdf.attach`

## Condições para os próximos lotes

1. Matemática: auditar schemas/capacidades no pin e reconhecer callbacks reais; completar pontes internas com L0 antes de código, sem tornar nomes discriminadores. Algumas famílias têm ABI engine; conservar cada uma. Não confundir helpers que retornam conteúdo com funções de elemento.
2. Formatos: `format.rs` registra svg/png/pdf/html/bundle com o mesmo callback `native_format`. A omissão não pode ser corrigida só comparando esse ponteiro: é preciso primeiro definir identidade interna distinta no L0 e revalidar a construção rejeitada. Não usar `Func.name()` para encobrir a colisão. PDF raiz, artifact/attach e HTML também precisam de seus schemas/identidades auditados; aliases devem conservar as funções efetivas.
3. Counter: os quinze controles deste lote continuam com a admissão anterior, separada em `stdlib/counter.rs`; o metadado de locatabilidade agora existe, mas não autoriza declarar counter completo. Auditar também bare selector, query e indexação, sem inferir suporte por `.where()` ou por `Locatable`.
4. Catálogo anterior: as entradas históricas conservadas não receberam uma certificação geral de todos os campos/capacidades neste lote.
5. Outras frentes: FG1 (Figure/caption tipados e síntese/presença), RM1 (medidas relativas por consumer), SY1 (morfologia Symbol), RG1 (row groups pendente de ratificação), paint text e demais itens do relatório anterior seguem abertos. TS1/equality já materializados são preservados.

Correções de tabela interna/paridade seguem ADR-0127 em fluxo contínuo. Novo campo/trait/assinatura Rust pública, fase, compatibilidade ou default exigem proposta concreta antes da implementação. O objetivo global permanece ativo; este relatório encerra só o lote identificado.
