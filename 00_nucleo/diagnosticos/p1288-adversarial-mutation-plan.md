# P1288 — plano adversarial pré-candidato

## Papel e limite

Regime completo da skill `tekt-materializacao-segregada`. O executor
`/root/p1288_l0` atua neste turno somente como **adversário pré-candidato**.
Ele foi autor/auditor de Núcleo + L0 em turno anterior, sobreposição aqui
registrada; nunca foi autor do contrato, dos oráculos, da implementação ou do
veredito. O checkout compartilhado não oferece isolamento ambiental forte.

Leitura autorizada: skill e referências, Passo 1288, manifesto/contrato,
baseline vanilla final, runner/testes/baseline/receipt dos oráculos e receipt
L0. Candidato e harness da Fase A são proibidos e não serão lidos/executados.
Escrita limitada ao driver, log, plano e receipt adversariais.

## Método

O driver carrega as expectativas protegidas sem reescrevê-las, constrói um
controle sem violações e aplica A01–A22 isoladamente sobre cópias em memória.
Cada mutante válido deve produzir `Violated` com testemunha. A ordem forward e
reverse deve gerar o mesmo conjunto canônico. Duas mutações inválidas
(no-op/path fora do modelo) ficam fora do denominador. Os quatro casos opacos
congelados são os únicos aceitos como `Unknown`.

O runner congelado é reutilizado para carregar/verificar inputs e seus casos
são citados como evidência. Obrigações sem API decisória isolada no runner —
ativação implícita por target, unicidade de MCID, órfãos de ParentTree, lattice
de classificação e drift direto de Núcleo/L0 — recebem meta-gates derivados
do manifesto/contrato. Isso é deliberadamente explícito: não se atribui ao
runner isolado cobertura que ele não expõe.

## Matriz A01–A22

| Ataque | Mutação | Testemunha mínima |
|---|---|---|
| A01 | HTML default on | perfil default |
| A02 | target ativa feature | ortogonalidade O01 |
| A03 | a11y default on | gate default do trio |
| A04 | html ativa a11y | perfil html |
| A05 | a11y ativa html | perfil a11y |
| A06 | flag aceita e ignorada | trio ativo ausente |
| A07 | só data-cell | trio indivisível |
| A08 | binding sem feature | gate default |
| A09 | stub devolve body | metadata não sobrevive |
| A10 | summary perdido | `/Summary` ausente |
| A11 | Header vira Data | TH vira TD |
| A12 | Data em header promovida | TD explícita vira TH |
| A13 | row/column trocados | scopes divergentes |
| A14 | level ignorado/zero aceito | relação/diagnóstico divergente |
| A15 | MCID duplicado | duplicação ou falta de reset |
| A16 | ParentTree órfão | referência incompleta |
| A17 | tags no modo disabled | estrutura residual |
| A18 | metadata muda visual/texto | par congelado diverge |
| A19 | disabled ganha crédito | lattice violado |
| A20 | flag desconhecida vira disabled | erro ativo escondido |
| A21 | forward/reverse divergem | canonicalização diverge |
| A22 | drift protegido ignorado | hash direto não bloqueia |

## Gate

`mutation_score = mortos válidos / mutantes válidos`; exigido `1.0`. Mutante
válido sobrevivente bloqueia o selo. O log persistente deve registrar hashes
protegidos antes/depois e permanecer evidência, nunca veredito de refinamento.
