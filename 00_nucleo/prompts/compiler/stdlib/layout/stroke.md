# L0 — stdlib/layout/stroke — stroke
Hash do Código: a95df4df

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stroke-paint-inputs.toml sha256:eef6bb1d6fb79379c1e42e22ef305bc31d31dab01539ba4e9fcffe49587c3c4f

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/stroke.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contém a lógica desta família em `01_core/src/compiler/stdlib/layout.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem nova prova bilateral contra o vanilla. As decisões históricas, medidas e qualificações do pin continuam em `00_nucleo/prompts/compiler/stdlib/layout.md`.

## Contrato de linguagem

stroke preserva forma posicional e named, paint, thickness, cap, join, dash, miter-limit e overhang; miter-limit exige valor finito positivo.

A extração preserva assinatura, identidade de função, sintaxe, morfologia, erros e precedência de validação. O hub e o registro continuam a encaminhar chamadas estaticamente, sem duplicação dos algoritmos. Não há mudança de contrato público, comportamento por defeito ou fase do pipeline.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. A paridade é da linguagem, segundo ADR-0107/0108; a mecânica pode divergir. Owner exclusivo conforme ADR-0129/0130.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Construir Value pelo From<Stroke> existente; ao receber Value::Stroke, produzir Stroke próprio para completar componentes posicionais, sem mutar o payload compartilhado. Preservar parsed fields, specified, defaults, validação e precedência de erros.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.

## Contornos com Paint e dicionário uniforme — 2026-10-14

### Medição anterior à decisão

before/results.json.gz no diagnóstico stroke-paint-2026-10-14 conserva HEAD,
working tree, diffstat, UTC, fontes/owners, comandos e hashes dos dois binários.
layout/stroke.rs:53-63 limita named paint a Color; shapes/paint.rs:45-64 entrega
os demais paints ao caster legado que só aceita Length/Color/Stroke;
shapes/rect.rs:40-43 descarta todos os Dict. O controle uniforme renderiza 1pt
preto em Tekt; named Gradient/Tiling/Auto e circle Gradient são rejeitados.
O binário vanilla pinado 9f2b6e871 aceita esses valores. Dict vazio desenha
contorno preto em Tekt e nenhum contorno no vanilla. A fonte pinada stroke.rs:
236-243 e sides.rs:238-258 distingue Paint smart, dict vazio e dict uniforme.
A medição de square(size:...) falha por dimensão no Tekt e não prova cast de
stroke; suas dimensões e defaults permanecem abertos, sem mudar este contrato.

### Responsabilidade e aceitação

O named paint usa o mesmo cast de Paint que o campo paint do dicionário
posicional: Color/Gradient/Tiling preservados; auto mantém specified.paint=false
e o fallback interno preto existente. Omissão também mantém esse slot não
especificado. None e outros tipos são rejeitados com a mensagem medida
"expected color, gradient, tiling, or auto, found <tipo vanilla>" e span recebido.
Extrair helper privado neste owner e usá-lo nos dois caminhos; não duplicar casts.
Não mudar validação de thickness/cap/join/dash/miter-limit, overhang ou mistura
positional/named nesta correção. Esses campos possuem diferenças legadas ainda
não encerradas. Nenhuma nova API pública ou entidade.

Correção interna de paridade ADR-0127 em fluxo contínuo: L0 primeiro, RED→GREEN,
resselo restrito, build, suíte e linhagem. Aceitação preserva significado da
pintura e sua forma disponível; igualdade binária com vanilla não é requisito.
Recolher PDF/raster e erros, conservando as divergências não corrigidas.
