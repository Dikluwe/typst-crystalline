# P1312 — revisão prévia de escopo

Revisor `/root/p1312_review`; regime A/B executado sem atestação de isolamento
técnico. Contexto recebido: atribuição explícita do papel, escopo proposto e
paths de baseline/medição/L0, sem candidato P1312. Escrita limitada a
`p1312-review*` e `p1312-verification.json`; nenhum write em produto, L0 ou
oráculos. Skill e ambas as referências lidas integralmente; CLAUDE raiz/core
e ADRs 0107, 0108, 0127, 0129 lidas. Busca na pasta ADR não encontrou ADR local
de materialização segregada. O passo exato P1312 foi autorizado pelo root e
lido; não foram lidos passos históricos nem a pasta context.

Baseline: HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree P1311
não commitado às `2026-09-08T01:26:34.909020+00:00`, com diff/stat integral em
`p1312-baseline.json`, SHA-256
`0597c75b13da999990886b32577b563bab330d8d1be4e5bf85e97ff0f03588c5`.
Medição fresca SHA-256
`d986f593b9d8ca8358d3bf8fc5303eade65bd47382dec0424c3cd3d9d54e07c1`.
Na leitura prévia ao candidato, o hash de loading.rs ainda coincide com o
baseline: `a4c574a7fcf28ee579f079a0473d56a81ebf8692aab91fd56e68bd957c407ca1`.

A fonte ratificada `a51e02804`, `loading/read.rs:24-29`, declara
`Spanned<PathOrStr>`; `loading/csv.rs:27-31` declara `Spanned<DataSource>`.
O cast PathOrStr (`foundations/path.rs:216-224`) inclui Str; Str converte
Symbol (`foundations/value.rs:632-637`). A medição bilateral confirma tanto
o erro PathOrStr de read(Bytes) como sucesso CSV(Bytes), e a tentativa de
leitura Unicode de read(Symbol) no vanilla. Portanto a coorte read/csv
histórica precisa da retificação: read-only é recorte defensável, nunca
fechamento da coorte inteira. CSV deve permanecer integralmente preservado.

O amendment P1312 em loading.md foi lido antes do candidato. Ele especifica
texto, tipo longo, origem da primeira ocorrência posicional, detached legítimo,
traces já existentes, preservação dos demais validadores e helper privado
próprio de read. Proíbe comparação por fname/fixture/texto. Preserva a rejeição
de Symbol mas declara o delta de formatter como obrigação local não paritária.
Isso evita a falsa igualdade com o vanilla e exige expectativa separada.

Classificação ADR-0127: correção interna de diagnóstico em função existente,
sem nova assinatura, aceitação de tipo, modo, flag ou fase. Fluxo contínuo
L0-first + RED→GREEN e revalidação é compatível com o recorte. Não há aprovação
de novos encoders propostos no mesmo L0 nem de CSV/DataSource. O preflight
V15/V26 registrado tem exit 0 e `No violations found`.

Resultado desta fase: escopo e L0 aptos. GO funcional depende ainda de revisar
o freeze A/B, suas expectativas e o RED unitário anterior ao patch.
