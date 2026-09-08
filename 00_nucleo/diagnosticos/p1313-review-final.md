# P1313 — revisão final independente

**Veredito: `PASS_SCOPED`. Nenhum achado acionável pendente no recorte P1313.**

CSV aceita Bytes em RAM e o cast inválido usa nomenclatura pública e origem
do primeiro valor posicional, conforme o L0 aprovado pelo dono. A implementação
e as expectativas independentes conservam as fronteiras aprovadas. Este
veredito não fecha paridade geral CSV nem as dívidas explicitamente excluídas.

## Evidência, estado e reprodução

Revisor `/root/p1313_review`, 2026-09-08T12:02:49.357Z. HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado.
O índice continuava vazio na conferência final; HEAD não mudou.

Recibo próprio `p1313-review-evidence.json`, SHA-256
`d5f631589e068734db35d3afc05358a0dd08c9e38ee528d8c83a1f103a634add`.
Ele registra diff/stat, identidades, hashes dos recibos examinados,
recomparações e resultados. O verificador somente leitura é
`p1313-review-check.cjs`, SHA-256
`4844d0da50c0a141eb2db18ea247b9f070ae9e18c9cd5b79084ab4f6da1014a3`.

Reprodução no host, que possui o namespace RAM dos binários:

```sh
git show eb24cd657fc2333dc7ea5393f7cfebf8c7192d39:01_core/src/compiler/stdlib/loading.rs | node 00_nucleo/diagnosticos/p1313-review-check.cjs
```

A primeira tentativa no sandbox viu ENOENT para os executáveis RAM. Isso foi
resolvido executando a checagem somente leitura no host com
`sandbox_permissions=require_escalated`; candidato, baseline e vanilla
existem e seus hashes físicos conferem. Não houve remoção nem reconstrução
dos executáveis durante esta revisão.

Identidades finais verificadas:

- Owner: `a8ef0db2cd9531d6f86aa5c346c3860495a002a8244b4e9594cc9fd6f15b3468`.
- L0: `103a2035a2c4c534e3e905bf168ec50d06dc6c2fc484cfc362a74b22f7bf9c5c`;
  pin normativo A/B permanece `553294f33c131121926207fb203c1e4a4174b03e653afe39e5827b3a773fead3`.
- Binário candidato: `cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce`.

## Código e escopo

O baseline do owner foi reconstruído a partir do HEAD e do diff integral
registrado antes da implementação; seu SHA-256 corresponde exatamente ao
baseline. A comparação confirma que a produção restante é idêntica, exceto
helper CSV, escolha Bytes em native_csv, comentários pertinentes e linhagem.
O decoder inteiro, read, decoders P1310 e encoders estão preservados.

O helper novo aceita somente Path/Str/Bytes, mantém missing legado e rejeita
os demais tipos com nomenclatura canônica. Ele encontra a primeira ocorrência
posicional, conserva value_span e não inventa origem detached. O ramo Bytes
fica depois das opções e retorna diretamente pelo decoder puro: não chama
World, não resolve caminho e não antecipa leitura. Os caminhos Path/Str usam
a leitura existente. Não surgiram entidade, trait, assinatura pública Rust,
dependência, fase ou lógica de I/O no core.

Os testes locais verificam ausência de leitura nas rotas pertinentes,
precedência, origem sintética/detached e Bytes com UTF-8 inválido contra o
decoder legado inalterado. A retirada das asserções P1312 que exigiam rejeição
CSV corresponde à substituição aprovada desse contrato; controles read
permanecem. Os oráculos independentes históricos não foram alterados.

## Gates e recomparações

Build, workspace, lint, fmt, diff-check e preview de linhagem têm exit 0.
Os snapshots before/after desses gates correspondem ao diff final revisado.
Os snapshots de binário que ainda mostram o predecessor em lint/fmt não são
confundidos com o build candidato: esses gates verificam a fonte, cuja
identidade foi conferida separadamente.

O RED-r1 contém seis falhas de comportamento, sem falha de compilação.
O GREEN local anterior ao resselo não prova a identidade final, pois houve
ajuste de comentário naquele intervalo. A suíte workspace final cobre o
estado final: 6.639 testes passam, nenhum falha, três doctests antigos ficam
ignorados; os seis testes P1313 passam nessa execução. Lint tem zero erros,
240 warnings e 1.138 infos; não é ausência de avisos.

As 1.728 observações A/B foram recomparadas diretamente contra o freeze,
exigindo unicidade e completude por caso/perfil/ordem. Nenhuma divergência;
normal/repeat/reverse mantêm os resultados. Entradas e pin normativo do
freeze permanecem intactos. Symbol usa a expectativa normativa própria,
não uma alegação de igualdade vanilla.

Replays, recalculados a partir das observações:

| Comparação | Preservados | Deltas autorizados |
|---|---:|---:|
| P1310 contra P1312 | 1.032 | 4 |
| P1311 contra P1312 | 220 | 0 |
| P1312 | 264 | 16 |
| P1308 contra P1312 | 1.974 | 8 |

As mudanças correspondem exatamente às expressões CSV predeclaradas e às
expectativas congeladas. A recomparação própria P1308 confirma seu conjunto
de chaves e os oito deltas novos iguais ao vanilla pinado. Contra o oráculo
histórico P1308 continuam 1.938 Preserved, 44 Violated e zero Unknown:
36 deltas anteriores mais oito P1313. Esse resultado não é reescrito como
replay histórico totalmente verde.

A comparação física de 8.083 arquivos do baseline encontra alterações apenas
no owner CSV e no L0 correspondente. Os 211 artefatos anteriores pinados
permanecem intactos. `p1313-final-report.md` continua sendo a proposta
histórica; a execução está em `p1313-implementation-report.md`.

## Limites e independência

Regime A/B executado sem atestação de isolamento técnico. O testador recebeu
contexto novo e congelou expectativas sem ler source/diff/testes locais;
o revisor não editou os artefatos julgados. Filesystem compartilhado não é
isolamento técnico. Não há selo completo, mutation score ou equivalência
geral de Typst.

Parsing, opções, named, missing, excesso e duplicatas mantêm as políticas
legadas aprovadas. Symbol continua dívida separada. A prova de ausência de
World, Args sintético e UTF-8 inválido vem de revisão de código e testes
locais, não do corpus CLI. As alegações do relatório de implementação foram
conferidas dentro desses limites. A referência final ao presente parecer
pode substituir sua linha de espera sem alterar qualquer entrada congelada.
