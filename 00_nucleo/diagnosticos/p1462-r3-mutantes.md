# P1462 R3 — revisão adversarial focal de M12

**Regime:** protocolo completo, revisão causal limitada ao mutante M12.  
**Papel:** adversário pré-candidato; não autoriza implementação, teste RED ou selo.  
**Execução:** checkout compartilhado; executado sem atestação de isolamento.  
**Baseline:** commit `61796fa6e183c08207e3828cec6d2e43da1af846`, tree
`4890fb690d222c093e858130da00d2062ae514dd`.  
**Bundle R3:** `/tmp/p1462-r3-mutations`.  
**Entradas públicas novas:**
`p1462-r2-preseal.json` SHA-256
`8897c2e7cf244fd49023032f8a766315e8e67e01d25c36a3dd1d8c46d47a9b3f` e
`p1462-r2-mutation-results.json` SHA-256
`a2ab11a0e5d00389869a7235139e9f4811fb859107de1493913857d95c48fedc`.

Nenhum oráculo privado foi lido ou executado. Manifesto, contrato, L0, testes,
código produtivo, catálogo/runner/fixtures R2 e todo o bundle R2 permaneceram
somente leitura.

## 1. Causa pública e hipótese R3

O preseal R2 parou no foco M12 com
`reason_code = o8-pdf-public-observable-unchanged`. A instância R2 mudava
`PdfSerializationMode` e produzia bytes PDF distintos, mas texto e metadata
públicos congelados permaneciam iguais. Pela ADR-0107, a diferença era
mecânica e a instância era equivalente no fragmento O8; ela fica excluída do
denominador R3 e não recebe crédito retroativo.

A hipótese R3 é produzir um vazamento do eixo SVG para um observável público
do PDF, sem alterar os outros targets:

```text
document.format_options.svg.pretty == false -> conservar pdf_tags recebido
document.format_options.svg.pretty == true  -> PdfTags::Disabled
```

A API foi medida antes da receita. Em
`03_infra/src/export/mod.rs`, `PdfTags` possui exatamente `Enabled` e
`Disabled`, default `Enabled`, e é independente de `PdfSerializationMode`.
Em `04_wiring/src/main.rs`, ausência de flag escolhe `Enabled`. Em
`03_infra/src/pipeline.rs`, `compile_to_pdf_bytes_impl` recebe `pdf_tags` e o
propaga a todos os exporters PDF. Portanto a receita não inventa API.

## 2. Instância M12 R3

| Propriedade | Valor |
|---|---|
| fonte | `/tmp/p1462-r3-mutations/M12` |
| source tree SHA-256 | `320fcbdd3d2be58859094551259e40c59ee837be93298303447bab089da7d19b` |
| binário frozen | `/tmp/p1462-r3-mutations/artifacts/M12/typst` |
| binário SHA-256 | `c4c77f14e554ac7b6952f9f67d26f8c580dfd7c5885e3747f914fe2927ce2be8` |
| tamanho / modo | `80544376` bytes / `0775` |
| build isolado | exit `0`, `23809 ms` |
| classificação de construção | válido, compilável, não equivalente no observável público O8-PDF |

Existe um único arquivo diferente de P00:
`03_infra/src/pipeline.rs`. O SHA-256 P00 é
`636ff3c02aae5ea7a3027aab53715a995aac3b327c4ade2362bd64a5872b88af`;
o SHA-256 M12 R3 é
`16ed2805bcc17fbabfae6f8f6993eb78a1ebaff3ec4e5110380d45492cdd8c63`.
O delta único, logo após obter o `PagedDocument`, é:

```rust
let pdf_tags = if doc.format_options.svg.pretty {
    PdfTags::Disabled
} else {
    pdf_tags
};
```

### Delta discriminatório R2 → R3

| Revisão | arquivo mutado SHA-256 | efeito | observável público |
|---|---|---|---|
| M12 R2 | `0c976bfe08cd3dd6f91974f94ce1de0b7657e31dd107a67aad79a5388513ee18` | `pretty=true` mudava somente `PdfSerializationMode` | texto/metadata iguais; equivalente |
| M12 R3 | `16ed2805bcc17fbabfae6f8f6993eb78a1ebaff3ec4e5110380d45492cdd8c63` | `pretty=true` desativa tagging PDF | `pdfinfo` muda `Tagged: yes` para `Tagged: no` |

Não há segundo desvio, mudança de texto, erro injetado, alteração de PNG/HTML,
reescrita de `html.frame` ou mudança do ramo `false`.

## 3. Foco público de construção, sem classificação de kill

As fontes false/true de cada par diferem somente no booleano de
`format.svg.pretty`. Os quatro recortes foram executados antes de qualquer
campanha completa:

| Recorte | P00 false / true | M12 R3 false / true | Resultado de construção |
|---|---|---|---|
| O8-PDF | `Tagged: yes` / `yes` | `Tagged: yes` / `no` | witness público novo; texto extraído idêntico nos quatro |
| O8-PNG | mesmo SHA-256 `ade1db3038fb87d3f027f3388d2d97724f648159dea990fa9c9e95659f575ee3` | mesmo SHA-256 | controle preservado |
| O8-HTML | mesmo SHA-256 `1a097884091001310c1d30ede4a197a16b9c938fc97d688e433155a3eb74f7d6` | mesmo SHA-256 | controle preservado |
| O8-HTML-FRAME | mesmo SHA-256 `6fc4956510bac79da666bcc6c97e67d57d484fc4c9405dc987332b48577271fb` | mesmo SHA-256, uma raiz SVG | controle preservado |

No PDF, o texto extraído tem SHA-256
`53cf8e98fe620672c07308b494b85433683a91c4d89066be70715116f2816421`
nos quatro artefatos. O foco prova que a nova instância é discriminável pelo
observável público pedido; somente o verificador independente pode atribuir
`Violated`, `Survived` ou `Unknown`.

## 4. Continuidade das outras 17 classes

P00, M01–M11 e M13–M18 foram copiados por reflink seguro, nunca por hardlink
editável. Fontes e binários são byte-idênticos ao bundle R2. A fixtures R3
registra e o runner valida cada source tree, binário, modo, tamanho e delta
exato contra P00.

O catálogo discriminatório continua com exatamente 18 instâncias válidas:

```text
M01–M11 R3 = respectivas instâncias R2, byte-idênticas
M12 R3     = vazamento PdfTags descrito acima
M13–M18 R3 = respectivas instâncias R2, byte-idênticas
```

M15/M16 continuam semanticamente preservados e negativos de ownership. M17
continua compilável e rejeitável por V26. M18 continua usando a mesma fonte
`styled-root` nos dois lados. A correção focal de M12 não reabre essas classes.

## 5. Denominador, ordem, `Unknown` e budget

O denominador R3 é `18`: uma instância válida por M01–M18. A instância M12 R2
equivalente não entra; a instância M12 R3 entra. O único score selável continua:

```text
mutation_score = mutantes válidos rejeitados / 18 = 1.0
```

`Unknown`, `Pending`, equivalente e não compilável nunca matam mutante, nunca
aumentam o score e nunca autorizam selo. O runner oferece `direct`,
`direct_repeat` e `reverse`, com P00 primeiro. A campanha completa não foi
executada por este papel.

Preserva-se o budget público: 120 minutos, até quatro revisões, no máximo três
execuções focais por revisão e três execuções completas antes do selo. Esta
revisão consumiu uma receita M12, um build isolado e um conjunto focal O8 com
controles. Esgotamento do budget permanece inconclusivo. Duas revisões
consecutivas com o mesmo vetor e a mesma causa obrigam reabrir o desenho.

## 6. Atestação e limite

R3 foi executada em checkout e filesystem compartilhados, sem worktree ou
sandbox atestável por papel: **executado sem atestação de isolamento**. Hashes
provam identidade e encadeamento, não isolamento de capacidade. Este documento
atesta somente a construção adversarial e seus controles públicos; não sela o
contrato e não autoriza candidato.
