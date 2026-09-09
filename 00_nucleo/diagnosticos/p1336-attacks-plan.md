# P1336 — plano adversarial congelado antes de C

Regime A/B, executado sem atestação técnica de isolamento e sem refinement seal.
Autor `/root/p1336_attacks`; entradas: L0 integral, skill e suas duas referências,
CLAUDE raiz/core, manifesto e baseline. Nenhum candidato foi lido para este plano.
Manifesto SHA-256 `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`.
L0 normativo SHA-256 `debbd739edd88ceb66ddc3c6073439222431ff1ce6d98977bdd72c7021fcdaaa`.
Baseline SHA-256 `e81e034167a994ab3dd1c318a7dd7384cd18ef75e0d2b3c1c666eb26724e441f`.
Fonte anterior SHA-256 `3bdabb8fc381a670d340e1f86cf933f0f9a61e9445772bbf437d410bc7f45f90`.

## Expectativas e mutantes

Derivação: o L0 exige nomes distintos para Int/Str, âncora exata somente do
identificador em ambos, e preservação do comportamento baseline fora deles.
O runner `p1336-attacks-probe.py` congela fontes e saídas integrais vanilla/baseline
em `p1336-attacks-oracles.json`, com identidades, comandos, UTC e custo.
Casos negativos Int/Str usam vanilla como expectativa; Bool e namespaces usam
baseline, mesmo quando baseline diverge do vanilla. Sucessos de métodos e namespace
preservam o baseline. O lookup puro terá o span recebido intacto, conforme testes
independentes congelados; nenhum teste será corrigido pelo adversário.

| Família | Mutação semântica produtiva prevista | Testemunha |
|---|---|---|
| M1 | Regredir exclusivamente o nome diagnóstico de instâncias Int de integer para int | i-direct / i-alias |
| M2 | Regredir exclusivamente o nome diagnóstico de instâncias Str de string para str | s-direct / s-alias |
| M3 | Fazer instância Int usar o span do acesso inteiro | i-direct / i-alias |
| M4 | Fazer instância Str usar o span do acesso inteiro | s-direct / s-alias |
| M5 | Generalizar ao Bool a âncora field-only destinada a Int/Str | bool-boundary |
| M6 | Confundir valores-tipo Int/Str com instâncias, rejeitando seus namespaces antes do lookup | int-namespace / str-namespace |

Cada família será uma alteração independente na fonte produtiva do candidato,
sem modificar testes, baseline ou oráculo. A forma concreta do patch será registrada
após C, sem alterar sua intenção. Compilar é requisito de validade; erro de build,
timeout ou fixture inválida resulta em Invalid/Unknown, nunca mutante eliminado.
Rejeição exige teste focal executado e falha discriminatória correspondente, com
controle candidato sem mutação aprovado. Não haverá crédito por score histórico.

## Cópia e orçamento

Após aviso C do operador e congelamento dos testes, criar workspace exclusivo em
`/tmp/p1336-attacks-*` por cópia regular/reflink de Cargo.toml, Cargo.lock, .cargo e
camadas produtivas necessárias. Nunca copiar por hardlinks mutáveis, nunca editar
fonte principal, nunca percorrer materialization/context. Cache target dedicado,
copiado sem hardlinks de target autorizado quando Cargo do operador estiver parado.
Um workspace de execução sequencial pode receber cada mutante sobre a cópia intacta
de C; todas as seis fontes, patches e saídas são preservadas em diretórios próprios.
Build/test `cargo test -p typst-core --lib p1336_tests` com target exclusivo, depois
de coordenação explícita de janela Cargo. Casos CLI congelados servem como referência
independente e a suíte congelada do testador como discriminador do produto.

Budget inicial: seis famílias, uma rodada focal de candidato e seis de mutantes;
até duas revisões instrumentais por causa, sem ajustar expectativas. Máximo oito
famílias somente mediante revisão de método registrada. Registrar argv, SHA de C,
SHA de cada fonte/patch, testes preservados, baseline, UTC, duração e exit/stdout/stderr.
Manter todos os outputs, inclusive builds inválidos. Reviewer independente julga
validade dos mutantes, testemunhas e limites do resultado.
