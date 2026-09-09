# P1335 — plano independente de revisão e ataques congelados

Congelamento anterior ao checker e aos resultados julgados: 2026-09-09T16:48:11Z.
Executor: `/root/p1335_review`. HEAD observado
`d31047d7b8af7837c84adae4ded3d2ff50c62093`; a árvore não commitada é a
baseline efetiva a receber do operador. Não atribuir resultados novos a HEAD sozinho.

Regime: revisão de auditoria com autoria separada e ataques somente em cópias
de dados do auditor, sem mutantes de produto. Skill tekt-materializacao-segregada
lida integralmente com ambas as referências. Não foi localizada ADR de segregação
em `00_nucleo/adr/`. Executado sem atestação técnica de isolamento: ferramentas e
filesystem compartilhados não impõem tecnicamente a allowlist declarada.

Entradas permitidas: somente o passo 1335 explicitamente autorizado, skill/referências,
ADRs, produto/L0 em leitura, diagnósticos históricos pertinentes e dados novos P1335.
Escritas: somente novos `00_nucleo/diagnosticos/p1335-review-*`. Não editar nenhum
catálogo, runner, inventário, ledger ou resultado julgado, nem produto/L0, nem evidências
históricas. Contexto herdado: instruções do repositório e delegação de revisão; nenhum
resultado novo foi recebido antes deste plano. Baseline/manifesto do operador serão
fixados como entradas antes do julgamento. A primeira chamada de status não aplicou
pathspec de exclusão e expôs nomes de arquivos restritos; não houve leitura desses
arquivos. As próximas consultas excluem essas pastas.

## Entradas normativas congeladas

| Arquivo | SHA-256 |
| --- | --- |
| `00_nucleo/materialization/typst-passo-1335.md` | `35bdf054a87ea41911aa4a1697e93145814b09fb770f97d6383be468cafc5913` |
| skill `SKILL.md` | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` |
| skill `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| skill `references/artefatos-e-gates.md` | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` |
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0127 | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |

Os controles finais P1322 e sua política estrita de query exit 2 podem servir de método.
Seus números e vereditos não são resultados P1335. A revisão atual confere todos os
canais desde o início; JSON/DOM igual com hints diferentes não recebe MATCH integral.

## Obrigações do julgamento

1. União bilateral nova default/html, IDs históricos reconciliados um a um,
   unicidade e expansão Symbol derivada de rotas reais sem recombinação artificial.
2. Produto/binários/fixtures/argv/cwd autenticados; quatro perfis efetivos,
   ordens normal/repeat/reverse recompostas por identidade; stdout/stderr/exit completos.
3. Recalcular classes a partir dos canais brutos. Missing/timeout/crash/identidade
   ambígua/parser opaco são Unknown. Exit 2 somente pode representar rejeição pública
   quando comando e transcript específicos completos a demonstram, com controles.
4. Denominador principal catálogo × perfis; repetições e suplementos separados;
   ajuste de extensões exige L0 atual e Unknown não desaparece do denominador.
5. Presença/kind/repr não fecha função. Cada fechamento P1323–P1334 exige testemunha
   reexecutada com alegação, residual, identidade e expectativas históricas reconciliados.
6. MATCH histórico divergente exige transição e controle de localização/adapter antes
   de regressão de produto; testemunhas atuais fundamentam causa, owner e refutação.
7. Prioridade P1335 §7 e todos os desempates devem ser reconstituíveis. Contagem de
   owners inclui transporte de origem, trace e fachada necessários ao observável inteiro.
   Unknown obrigatório, integridade inválida ou instabilidade bloqueiam seleção.
8. Gates novos e preservação byte a byte confrontados com baseline efetiva. Warnings,
   testes ignorados e falhas preexistentes ficam explícitos.

## Ataques e expectativas anteriores ao checker

Todos são transformações negativas em cópias de dados reais com hashes de origem.
Sem precondição demonstrada, registrar NOT_EXECUTED. Violated exige testemunha;
opaco honesto permanece Unknown. Nenhum ataque conta como mutation score do produto.

| ID | Transformação | Controle e resultado esperado |
| --- | --- | --- |
| R01 | Omitir ID histórico da reconciliação | Reconciliação completa Preserved; omissão Violated |
| R02 | Duplicar ID no catálogo/célula | IDs únicos Preserved; duplicação Violated |
| R03 | Adicionar modificador Symbol recombinado sem rota real | Expansão observada Preserved; ficção Violated |
| R04 | Trocar identidade de binário | Identidades pinadas Preserved; troca Violated |
| R05 | Trocar label de perfil mantendo argv | Quatro perfis autênticos Preserved; troca Violated |
| R06 | Apagar flag efetiva requerida | Argv completo Preserved; flag ausente Violated |
| R07 | Apagar stderr/hint não vazio conservando MATCH | Canal integral Preserved; adulteração Violated |
| R08 | Converter Unknown em MATCH | Unknown honesto Unknown; promoção Violated |
| R09 | Declarar fechamento apenas por presença | Limite de inventário Preserved; fechamento fictício Violated |
| R10 | Omitir transição de MATCH histórico para divergência | Transições completas Preserved; perda Violated |
| R11 | Incluir suplemento no denominador principal | Denominadores separados Preserved; inflação Violated |
| R12 | Subtrair extensão sem fundamento L0 atual | Extensão fundamentada ou dívida explícita Preserved; ajuste indevido Violated |
| R13 | Selecionar coorte inferior ignorando prioridade | Ordenação fundamentada Preserved; seleção inferior Violated |
| R14 | Retirar owner necessário ao observável completo | Owners demonstrados Preserved; subcontagem Violated |

Controles adicionais: reordenar registros preserva resultado; ausência bilateral não
é membro unilateral ausente; igualdade de projeção não apaga canais diferentes;
rejeição CLI genérica, truncada ou crash não entra na exceção pública específica.
Regras semânticas R09/R12–R14 incluem inspeção humana/da fonte pelo revisor; um
predicado textual não prova por si só intenção ou suficiência arquitetural.

## Budget e sucessão

Até duas revisões focais por causa antes de rever método; somente após recorte corrigido
passar cabe repetir o conjunto de controles. Mudança do checker não exige nova matriz
bilateral. Registrar custo, UTC, vetor de classificações e delta por revisão. Duas
revisões sem ganho na mesma causa interrompem a calibragem e deixam a causa inconclusiva.
Ordem: plano → baseline/manifesto → leitor de schemas → controles/ataques → revisão
focal → classificação/seleção/gates → recibo final com hashes. Dados julgados sempre
somente leitura. Não escolher previamente math, calc.abs, CSV nem outra família.
