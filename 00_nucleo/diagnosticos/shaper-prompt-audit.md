# Auditoria dos prompts e consumers de shaper

Revistos o hub `infra/shaper.md`, seus sete filhos e os consumers correspondentes.
Encontrados quatro defeitos adicionais demonstrados: dois no tracking da chave,
um no scoring de fallback e um no respeito às fronteiras de script. Somente
sondas e este diagnóstico foram adicionados; nenhuma correção produtiva ou de L0
foi feita nesta auditoria. A correção anterior dos eixos foi preservada.

## Evidência e classificação

### 1. Arredondamento de tracking: erro prescrito pelo L0

`00_nucleo/prompts/infra/shaper/cache.md` ainda exige quatro casas para as partes
absoluta/em do tracking. `03_infra/src/shaper/cache.rs:47` obedece. Em Libertinus
Serif, texto `ab`, tamanho 12pt, tracking 0.00599pt versus 0.00601pt: ambos viram
`0.0060`, mas o avanço fresco do primeiro glifo é 457 versus 458 unidades da fonte.
No documento AB ambos usam 457; no BA ambos usam 458. É outro caso real da mesma
classe de colisão corrigida para os eixos, independente de fonte variável.

Origem: contrato L0 incorreto, materializado no código. Owners para correção:
cache para identidade; shaping para conversão/aplicação. Regressão necessária:
resultados mistos devem coincidir com frescos em ambas as ordens. Não basta
simplesmente aumentar a quantidade de casas decimais.

### 2. Tamanho ausente: chave não cobre toda a fórmula

`03_infra/src/shaper/shaping.rs:292` resolve tracking em pontos pelo tamanho e
converte para unidades da fonte, dividindo pelo tamanho. A chave de cache não
inclui tamanho nem o incremento efetivo que será aplicado aos glifos.

Com Libertinus Serif e `ab`, tracking absoluto de 1pt, tamanhos 12/24pt produzem
avanços frescos 540/499 para o primeiro glifo. AB produz 540/540; BA produz
499/499. O tamanho fica preservado no estilo emitido, mas os glifos reutilizados
já contêm o tracking calculado para o outro tamanho.

Origem: contrato da chave incompleto e implementação seguindo essa lista
incompleta. Não é necessário separar todo cache por tamanho se a identidade
puder representar exatamente todos os ajustes efetivos; essa decisão precisa
ser especificada e validada antes da implementação. Testar mudança de tamanho
com tracking absoluto, tracking em em e controles de reutilização correta.

### 3. Fallback: índices locais usados como globais

O L0 `infra/shaper/font_selection.md` distingue explicitamente índice local de
slot global e exige scoring coerente com FontBook. `covering_all` retorna
`primary.len()+posição_local` (`03_infra/src/shaper/font_selection.rs:159`).
`covering_run` passa esses números diretamente a `FontBook::select_fallback`
(`:221`); esta função lê `self.infos[id]` (`01_core/src/entities/font_book.rs:382`),
portanto espera slots globais. A tradução posterior em `get` não corrige o scoring
já realizado contra os metadados errados.

Sonda controlada: primária slot 0, candidatos globais 5 e 6, índices locais 1 e 2.
As duas candidatas usam a mesma face CJK, com mesma cobertura e comprimento de run;
metadados controlados tornam o slot 6 vencedor do scoring global. Metadados decoy
nos slots 1/2 fazem o consumer emitir a família do slot 5. A API pública de
FontBook escolhe `Primary CJK` (6); shape_document emite `Far Fallback With Long
Name` (5). A fixture sintética isola identidade e scoring, não afirma frequência
no catálogo de sistema nem paridade com um output vanilla.

Origem: erro de implementação contra um L0 correto. Regressão necessária:
catálogo esparso, metadata decoy e igualdade de cobertura/comprimento; verificar
slot/família efetiva, não só cardinalidade do vetor ou ausência de panic.

### 4. Fronteiras de script apagadas quando a fonte é a mesma

O L0 `infra/shaper/font_selection.md` manda dividir em mudança de script ou fonte.
`current_script` começa Unknown (`font_selection.rs:363`); só é atualizado dentro
de `script_changed` (`:396`). Unknown é compatível com todos os scripts (`:452`),
logo essa condição não estabelece o script inicial. Mesmo quando
`next_script_boundary` limita o trecho, a extensão por mesma candidata (`:400`)
concatena os trechos de scripts distintos.

Sonda: Libertinus Serif, `abαβ`, mesma face cobrindo todos os glifos (nenhum glyph
id zero). O consumer emite um único TextShaped com texto `abαβ`, em vez de respeitar
as fronteiras Latin/Greek previstas no seu próprio L0. Este é um controle do
contrato interno explícito, não um critério novo de paridade mecânica com vanilla.
Não foi demonstrado aqui que o PDF desses caracteres específicos muda; o defeito
comprovado é a quebra da segmentação prometida. Regressão: scripts distintos
suportados por uma só face, sem confundir troca de fonte com troca de script.

Origem: erro de implementação contra um L0 correto.

## Demais cláusulas e limites

| Owner | Resultado desta revisão |
|---|---|
| shaper (hub) | Delega às unidades e conserva visibilidade. Não identificado novo erro de contrato no hub. |
| bidi | Consumer acompanha parágrafos, ordem visual e exclusão de separadores B declarados. Política de direção automática é explícita. Não foi feita nova bateria de paridade bidi. |
| cache | Eixos corrigidos anteriormente preservados; tracking com dois defeitos demonstrados acima. Script matemático não faz parte da chave e permanece suspeita não demonstrada. |
| font_selection | Dois defeitos demonstrados acima; fallback lazy ainda confia no resultado de candidates_for_char, enquanto o L0 fala em confirmação por glyph lookup. SystemWorld hoje calcula cobertura exata; não foi demonstrado falhamento dessa confiança em um catálogo real nesta sonda. |
| shaping | Além da dependência de tamanho, existem divergências documentais sobre clone da face base e uso de língua, detalhadas abaixo. |
| reconciliation | Consumer acompanha a política explicitamente não recursiva, tolerância 0.5pt, linha matemática inteira excluída e fórmulas RTL. A auditoria não prova paridade de layout para todos os wrappers ou baselines próximas. |
| traversal | Consumer visita background/items/foreground e wrappers Group/Link/Semantic; TextShaped não é reshapado. Não encontrado novo defeito na leitura desses contratos. |
| tests | Cobertura atual não isolava os contraexemplos encontrados. Os testes existentes não são prova de ausência de defeitos. |

O L0 shaping exige clonar a face base em cache, mas `shaping.rs:234` constrói uma
nova face rustybuzz nos misses; FaceCache conserva uma face ttf-parser para
seleção, não uma base rustybuzz clonada para essa operação. É divergência factual
entre L0 e consumer, sem custo de tempo atribuído. Não deve ser usada para dizer
que o parse já está eliminado.

O L0 shaping fala em preservar língua; o consumer conserva style.lang no estilo
emitido, porém nunca o aplica ao UnicodeBuffer nas rotas de medida/emissão.
O vanilla pinado aplica `buffer.set_language(language(ctx.styles))` em
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980`. Essa diferença
é estática e a cláusula L0 precisa distinguir preservar metadado de configurar o
shaper. Uma fonte com substituição locl dependente de língua, texto e idiomas
que produzam diferenças no oráculo é necessária para provar impacto. Não foi
executada essa experiência aqui.

A emissão força script math mesmo quando ssty é None, mas a chave só identifica
ssty. Foram testadas seis strings em quatro fontes (24 controles normal/math),
sem diferença entre resultados frescos. São controles não discriminantes: não
refutam a suspeita. O mesmo vale para língua na chave caso seu uso seja ativado.

O teste existente de scoring P838 usa uma primária e candidatos slots 1/2, onde
índices locais coincidem com globais; não alcança o erro esparso. Os testes de
fronteiras de script usam fontes diferentes para scripts diferentes, sem isolar
o caso da mesma fonte. Tracking existente verifica aumento de avanço, sem os
pares de precisão ou tamanho que colidem. Algumas fixtures dependem de fontes
do sistema e podem fazer SKIP. Isso explica lacunas de detecção, não invalida
os controles que esses testes de fato exercitam.

Não foram alteradas decisões de direção, tracking por cluster, features da
medição/emissão, fallback matemático, reconciliação, defaults ou fases. Esta
revisão não autoriza copiar mecânica do vanilla nem declarar o shaper todo correto.
Nenhum tempo novo ou ganho de desempenho foi medido.

## Protocolo e proveniência

`shaper-prompt-audit/README.md` descreve os controles. `src/main.rs` usa apenas APIs
públicas atuais; World sintético restrito ao controle de fontes não oferece I/O
de compilação completo. Fontes reais/hashes em `inputs.json`. Build release offline,
repetido com --locked, flags e perfil em `provenance.json`; consumers/L0s/fontes
pinadas têm fingerprints iguais antes/depois. HEAD e estado exato não commitado
em provenance/status/incoming.patch. A patch tracked permanece idêntica.

Execução inicial final em UTC `2026-09-27T16:53:13.621021+00:00`, seguida de duas
repetições do executável arquivado que produziram resultados idênticos.
`results.json` conserva glifos/avanços e ordem; `repetition-{1,2}.json` conserva as
repetições. `preliminary/` preserva a sonda anterior ao controle de segmentação e
uma falha preparatória de compilação, sem misturar esses outputs com o resultado
final. `completion-audit.json` registra integridade e hash deste relatório;
`MANIFEST.sha256` sela a evidência nova. Datasets e relatórios anteriores ficaram
intactos, inclusive a correção dos eixos.

Verificação somente de leitura:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/shaper-prompt-audit/verification/verify.py
```

Próximo trabalho concreto: especificar a identidade efetiva de tracking e
corrigir/retestar essas duas colisões; corrigir a tradução local/global e a
segmentação com controles independentes. Língua, script math e cobertura ficam
pendências separadas até terem experiências discriminantes ou contrato esclarecido.
