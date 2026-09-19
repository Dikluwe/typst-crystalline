# P1462 — ataques adversariais finais de `format.svg.pretty`

**Estado:** CONCLUSIVO — nenhuma refutação do candidato

**Papel:** adversário final segregado; não autor do contrato, teste mantido ou
implementação

**Atestação:** executado sem atestação de isolamento

## 1. Estado atacado e limites de autoridade

A campanha ocorreu em 2026-09-19 sobre o commit
`3a997a3cd303bfe7e225a926253ecea2e633b883`, tree
`6f2d5a1e2c4491cdfc03e3ac0e46a13593475b76`. Antes de criar este relatório,
`git status --porcelain=v1` e `git diff HEAD --stat` estavam vazios. O checkout
e o filesystem são compartilhados; a separação é de papel, ordem causal e
artefato, não isolamento técnico.

Foram tratados como entradas imutáveis:

| Entrada | SHA-256 observado |
|---|---|
| contrato R2 | `69cfdc0de39564cf542e2ca7fc99bcb7b80752d10bc8cdce3c809c1d9cf02fa2` |
| recovery3 checkpoint | `ab771563f013604dc7d0b24ca6f7e35ac0e4714d486265a8c694c3252ddb61ac` |
| recovery3 preseal | `789b1f1c7e2a2c407edb36041588269844df2b88ff667aa6332ced5016d1a08c` |
| correção P1357 em Markdown | `4431aeedb8be0d64aff419210a618581e9aba58e7770bd8711d0b585d4ce65c8` |
| correção P1357 canônica JSON | `c2b9f2b1b1a568d1cbb52b055c0b997ab0a709239f8e72faea1f5e5e358c1662` |
| recibo RED | `3e7191029cdf07a7d1e77bf2f843cf21a77be9885b7e19c91993a9cae64696d3` |
| ADR-0131 | `2ba422df0147a6f6469e869d91aa07e3ddac102432554c800da9d03e65fad3cc` |

Nenhum código produtivo, Prompt L0, contrato, oráculo, teste mantido ou
artefato anterior foi editado. Esta etapa não reexecutou os binários mutantes
congelados: conserva as classificações seladas de P00/M03/M12/M13 e repete no
candidato, pelo teste A/B mantido, exatamente as fronteiras públicas que os
discriminam. Portanto não cria um segundo mutation score nem reclassifica o
vetor histórico.

## 2. Integridade causal do teste A/B

O recibo RED congela o arquivo original com SHA-256
`e1c95a932cb4304192188a81cc30f220287a90fe5d94e2f542272b2fdecaa49f`.
O arquivo commitado mede hoje
`f48c24a73861affc40bd7668e84a5d1b1e540e7fc0a1f35365c992197c284000`.
A diferença é exclusivamente mecânica nas duas linhas de linhagem:

1. `//` foi corrigido para `//!`, para o linter reconhecer o header;
2. o hash bruto completo do Prompt foi substituído pelo selo curto
   `08b20f8b` calculado por `crystalline-lint --fix-hashes`.

O corpo semântico, das linhas 3 ao fim, mede
`7722b12d6a3b4ed59540fa26ef24a4ebb9a2efd82141d941b60089a520c4c1f3`.
Ele é idêntico no commit de congelamento `6a185d6c5` e no candidato. Como
checagem independente, reconstruir em memória as duas linhas originais
registradas no recibo sobre o corpo atual reproduziu exatamente o SHA-256
congelado `e1c95a...`; nenhum expected, fixture, observador ou assert foi
adaptado depois do candidato.

## 3. Execuções reproduzíveis

### 3.1 Repetição integral do teste mantido

Janela local: `2026-09-19T17:50:24-03:00` a
`2026-09-19T17:51:34-03:00`.

```text
cargo test -p typst-wiring --test p1462_format_svg -- --nocapture
```

Resultado: exit `0`; `7 passed; 0 failed; 0 ignored`; duração reportada dos
testes `45.94s`. Binário observado após a execução:
`target/debug/typst`, SHA-256
`b48ce8aef06af7ffd12fc69ea8523391fbb83f6f8712b7f9e85ed13af917ee66`.
Houve espera normal pelo lock do diretório de build antes da compilação; ela
terminou, não perdeu observável e, pela ADR-0131, não é `Unknown` semântico.

Os sete casos preservaram:

- identidade/default e rejeição de construção manual;
- erros normais da set rule;
- root positivo e `block`/`box` negativos sem artefato;
- default, vazio neutro, as duas ordens lexicais e a precedência
  `caller > documento > false`;
- XML válido e morfologia normalizada idêntica entre compacto e pretty da
  mesma fonte;
- neutralidade de PDF, PNG, HTML e `html.frame`;
- ausência de siblings e aritmética canônica do overlay P1357.

### 3.2 Ataques internos fail-closed e de transporte

Em `2026-09-19T17:52:05-03:00`:

```text
cargo test -p typst-core p1462_ --lib -- --nocapture
```

Resultado: exit `0`; `7 passed; 0 failed`. Foram exercitados o valor interno
não booleano, dois containers distintos, ambos os sentidos de ordem,
transparência de `Styled` sem a chave, defaults e ausência estrita de siblings.

Em `2026-09-19T17:52:08-03:00`:

```text
cargo test -p typst-infra p1462_ --lib -- --nocapture
```

Resultado: exit `0`; `6 passed; 0 failed`. Foram exercitados construção
manual, nested real no pipeline, diagnósticos, vazio neutro, coleta no
documento e todas as classes da precedência `Option<bool>`.

Warnings preexistentes foram emitidos durante a compilação; nenhum deles é
falha P1462. Depois das execuções, `git diff --check` terminou com exit `0` e
sem saída.

## 4. Ataques dirigidos por risco

| Risco | Ataque final e testemunha | Classificação |
|---|---|---|
| P00, controle positivo | O teste mantido completo preservou todos os sete agregados do contrato; o checkpoint selado conserva P00 `Preserved`. | **Preservado** |
| M03, construção manual retorna sucesso | `public_identity_default_and_manual_construction` e o teste L3 observaram erro contendo exatamente `cannot be constructed manually`; `native_svg` retorna `Err` incondicional em `format.rs:20-27`. | **Violado** |
| M12, opção SVG vaza para outros targets | O8 comparou texto/metadata PDF, dimensões/pixels PNG, DOM HTML e DOM+SVG interno de `html.frame` entre documento false/true; tudo igual. O checkpoint selado conserva a testemunha inversa do mutante. | **Violado** |
| M13, sibling antecipado | `format.png`, `format.pdf` e `format.html` falharam como membros ausentes; a factory contém exatamente um filho (`format.rs:29-42,58-72`). | **Violado** |
| nested aceito ou artefato parcial | root pretty teve sucesso; `block` e `box` falharam com a mensagem congelada e sem artefato. | **Violado** |
| colapso precoce de `None` | caller ausente consultou o documento; `Some(false)` venceu documento true e `Some(true)` venceu documento false. | **Violado** |
| regra vazia escreve false | vazio isolado conservou default; `true → vazio` permaneceu pretty; `false → vazio` permaneceu compacto. | **Violado** |
| primeira decisão vence | `true → false` terminou compacto e `false → true` terminou pretty. | **Violado** |
| pretty altera XML | os dois SVGs da mesma fonte foram XML válido, com formas distintas e canonicalização igual. | **Violado** |

Aqui `Violado` significa que a hipótese mutante foi refutada. O candidato
preservou a obrigação correspondente. Não houve `Unknown` semântico.

### M13 e correção canônica P1357

A linha bruta `format.svg.pretty` continua sendo uma assimetria mecânica dos
enumeradores: o vanilla a guarda em `format.svg.params`; o cristalino a expõe
por `Func::namespace`. A correção canônica pinada exige o caminho literal,
owner `format.svg`, tipo bool, parâmetro vanilla named/settable/default false e
sonda bilateral false nos quatro perfis. O teste final voltou a provar no
candidato o default público false nos quatro perfis e a ausência dos três
siblings; a aritmética congelada conservou raw `43` e somente coalesceu esse
alias, produzindo `CRYSTALLINE_ONLY == 42` sem acrescentar `BOTH`. Nenhum
sibling é normalizado. Falta de qualquer precondição continua fail-closed.

Não se rodou outra campanha P1357 integral nesta etapa, porque os catálogos e
a sonda bilateral já estão pinados no recovery3 e o objetivo adversarial foi
refutar deriva do candidato nas precondições públicas, não recalcular o mesmo
inventário. Nenhum crédito novo é atribuído ao raw histórico.

## 5. Inspeção arquitetural

O coletor L1 é fail-closed em duas passagens com responsabilidades distintas:

- `format_options.rs:77-86` usa a travessia exaustiva de `Content::map_content`
  para validar e contar toda ocorrência da chave canônica, inclusive nested;
- `format_options.rs:48-69,88-97` percorre como root apenas `Sequence` e
  `Styled`, aplica last-write-wins e rejeita quando a contagem total excede a
  contagem de root;
- todo valor canônico é validado antes da decisão de fronteira
  (`format_options.rs:35-46`), portanto valor interno não booleano não pode ser
  mascarado pelo default;
- o pipeline coleta sobre o conteúdo realizado final e propaga o erro sem
  publicar documento parcial (`pipeline.rs:984-998`).

A fronteira de precedência também permanece fechada:

- L4 conserva o `Option<bool>` cru em `pretty_override` e só o passa ao braço
  SVG (`main.rs:650-651,728-748`);
- a entrada legada bool vira explicitamente `Some(pretty)`, enquanto a entrada
  documental usa `None` (`pipeline.rs:1862-1888`);
- o colapso ocorre uma única vez em
  `resolve_svg_pretty`, depois de o `PagedDocument` existir
  (`pipeline.rs:1891-1915`).

PDF e HTML continuam a interpretar somente a opção explícita do caller segundo
suas próprias superfícies; PNG não recebe `pretty`; a decisão documental SVG
não alcança esses braços. Não há estado global acumulado, e cada compilação
recalcula `DocumentFormatOptions` do próprio conteúdo realizado.

## 6. Veredito adversarial

O candidato P1462 preservou todas as obrigações observadas e refutou as classes
de defeito dirigidas por risco. P00 permanece **Preservado**; M03, M12 e M13
permanecem **Violados** no vetor selado e suas fronteiras públicas foram
repetidas com sucesso no candidato; `Unknown` semântico = `0`. Não foi
encontrado achado bloqueante nem não bloqueante no recorte P1462.

Este é um relatório de ataque, não o veredito final de certificação.
