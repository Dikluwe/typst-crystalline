# P1327-R2 — revisão da sucessão de escopo antes de C2

Regime A/B sem atestação técnica de isolamento. Revisor read-only sobre
produto/L0/oracles; escrito somente este relatório.

Baseline R2 `84056881f6481305cd530532849bf470837bbc966c4f7fd138b0094c46954b3e`
verificado. Registra working tree do HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, estado completo às
2026-09-09T11:05:40.323906 UTC, originais wiring, manifesto original,
corpus C1 Violated e revisão causal. O corpo de main.rs atual ainda é
igual ao original R2 excluído `@prompt-hash`: C2 não foi escrito.

Li integralmente o L0 wiring vigente, inclusive a nova seção P1327-R2, e
o núcleo `wiring/cli-observables.toml`. O novo texto mede primeiro as
24 falhas e suas fontes, depois especifica a única sucessão: `run_eval`
deve imprimir errors antes de warnings quando a avaliação retorna Err.
Preserva as ordens dentro de cada grupo, todos os blocos e Source, stdout
vazio e exit 1. Ok conserva warnings antes da serialização; falhas posteriores
de serialização/I/O não recebem nova política. Compile/query/watch/drain
genérico ficam explicitamente fora.

A decisão é compatível com o núcleo de observáveis CLI e com o L0 existente.
Os gates históricos de flags/APIs/watch em outras seções não se aplicam a
esse delta privado de apresentação. ADR-0127 contínuo aplica-se: é correção
de paridade diagnóstica, sem API, default deliberado de produto ou mudança
de fase/ordem do pipeline eval-layout. A ordem de impressão em stderr é
observable da língua, portanto não pode ser apagada do oracle.

O conjunto ampliado possui exatamente modules produtivo, tests test-only,
wiring produtivo e respectivos L0 1:1. A composição do novo owner tem acesso
a ambos os grupos e pode preservar formatter/sink sem novos carriers ou
owners. Isso resolve a causa medida; generalidade deve ser verificada com
warnings preexistentes, conforme L0, antes de considerar aceitação.

O wrapper aditivo `p1327-r2-record.py` mantém o recorder original imutável,
acrescenta apenas o par wiring ao allowlist e separa nomes de saída R2.
O baseline R2 foi comparado ao inventário final da suíte C1 antes de
continuar. O manifesto R2 ainda não estava disponível nesta inspeção;
esta revisão não atesta um manifesto que não leu.

Veredito L0/escopo: PASS para congelar sucessor manifesto e suplemento
independente antes de C2. O RED C1 integral já fornece testemunha causal
e deve continuar Violated como histórico. O novo corpus não pode adaptar
transcripts anteriores; precisa conservar controles Ok, erro sem warning,
warning sem erro e preservar a limitação de target/perfis já registrada.
