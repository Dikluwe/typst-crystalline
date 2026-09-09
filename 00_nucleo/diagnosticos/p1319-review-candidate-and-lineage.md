# P1319 — candidato e discrepância de linhagem reversa

Revisor `/root/p1319_review`, somente leitura dos artefatos julgados, regime
A/B sem atestação técnica de isolamento. Nenhuma correção de produto/L0 ou
ferramenta foi feita pelo revisor.

## Candidato examinado

Source SHA-256 `4518a5531a69163cf9f6474ca04cd22ac0906cd8e10fd98297cf93b9e9ba890e`,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
O diff produtivo introduz somente contexto privado Pure/Bytes/File, preserva
o parser único e a causa, consulta validade integral após erro, mantém
RootedPath até a formatação e recupera o primeiro value_span posicional.

O ramo Pure conserva detached e causa sem sufixo. Bytes conserva apresentação
P1318. File binário formata Project/Package e posição pelo helper vigente;
File textual não recebe a mudança. A resolução e leitura seguem o helper
existente, uma vez, com envelope de I/O igual. Não há include_path/source,
nova identidade externa, assinatura pública, crate ou fase.

Sem achado funcional bloqueante no diff. GREEN, A/B e demais gates continuam
necessários; este parecer não os antecipa.

## Medição de linhagem

Fonte atual sem a única linha `//! @prompt-hash 94cfda0e`:

```text
SHA-256 bf49e7b7a0874d81e24e2a0bdf23397fd504d4244099314a4cdaef0ee7a54edf
B esperado bf49e7b7
B registrado no L0 5249d235
```

Cálculo independente com Node crypto SHA-256, remoção ancorada apenas daquela
linha canônica, preservando demais bytes. O arquivo contém UTF-8 válido e
linhas LF. Logo não é colisão nem identidade esperada com o RED.

Comando somente leitura `crystalline-lint --checks v5,v15,v26 --fail-on
warning .` terminou exit 0, `No violations found`, apesar de B obsoleto.
Executável `/home/dikluwe/.cargo/bin/crystalline-lint`, SHA-256
`e47974fe903b225e96fec580040604d80380edda054a7b2187bbf5b18da9a30b`.
Isso demonstra que esse gate não basta para atestar a relação reversa no
estado examinado.

## Fonte da ferramenta e explicação causal

Fonte local da ferramenta em `/repos/Antigravity/tekt-linter`, HEAD
`49f46885fd03f753e9f1cf37e271d6baf30ed725`, árvore limpa na inspeção:

- `03_infra/hash_writer.rs:14-19`: declara SHA256[0..8] do source excluindo
  sua linha @prompt-hash e implementa exatamente esse cálculo.
- `03_infra/prompt_io.rs:150-233`: retira somente metadata canônica no header,
  preservando o resto dos bytes.
- `02_shell/fix_hashes.rs:293-300`: monta entradas somente para violações V5.
- `04_wiring/main.rs:810-838`: monta pares transacionais a partir dessas
  entradas; código sem V5 não entra no plano de atualização do B.

A hipótese causal é que, com A correto e norma L0 estável, não há V5 que
provoque a atualização do Hash do Código. Ela é coerente com o `Nothing to
fix` informado pelo root e o falso negativo reverso observado neste gate.
Não se afirma proveniência de build entre o binário instalado e o checkout
da ferramenta somente por estarem presentes; ambos estão identificados acima.

## Decisão

Não aceitar `Nothing to fix` como prova de coerência bidirecional. O resultado
contraria a exigência ADR-0129 para essa mensagem. A inconsistência concreta
do par P1319 precisa ser corrigida pelo autor e ambos os sentidos recalculados
antes do aceite final, preservando o hash normativo do freeze.

Uma correção da própria ferramenta exige seu fluxo/owners e não pertence a
este recorte CSV. Registrar a limitação em vez de alegar que lint detecta B
obsoleto. A observação acima não demonstra problema de V15/V26: ownership e
pins não foram alterados pelo candidato.

## Reparo r1 e GREEN auditados

O autor corrigiu somente a metadata B do L0 para `bf49e7b7`. Source continua
`4518a5531a69163cf9f6474ca04cd22ac0906cd8e10fd98297cf93b9e9ba890e`;
L0 raw passa a
`181f748041e4f8bec6f80a724930ffbeee5bb98d1ec52068047a296b00566c7d`.
Norma congelada permaneceu igual. Recibos recalculados e lidos:

- `p1319-lineage-before-repair.json`, SHA-256
  `3752b30ed395dfd2f5b4b99fcc8da3b2e5f6ecb9167d057a1b14ec2a11cde417`:
  preserva o falso negativo do linter e B incorreto.
- `p1319-lineage-verified-r1.json`, SHA-256
  `70f95c86e4175d610a074f03a1b2a03b55e5b749bc541a2f8bc8dbd4823a1570`:
  cálculo B explícito correto, norma igual e V5/V15/V26 exit 0.
- `p1319-unit-green-r1.json`, SHA-256
  `dfb6b5c94b35d541917bf10d986208b39cbbe82601f10e72913ceb583a7aade1`:
  mesmo comando do RED, 69 passaram, nenhuma falha/ignorado, fonte/L0
  estáveis antes/depois e target P1319.

Para verificar imutabilidade dos testes além da afirmação do autor, o revisor
reconstruiu a fonte RED em memória aplicando o diff do recibo ao `git show
HEAD:01_core/src/compiler/stdlib/loading.rs`. O SHA reproduziu exatamente
`9c647415dac61f26ac5a729bb9f5cfa80867bb85de83b76288d40e87f3a9152c`.
O módulo completo a partir de `#[cfg(test)]` é byte-idêntico ao candidato,
SHA-256 `9d89b7d2441cc4f5f76b5222abf9115f2ead0a8a2be081bdf3c6674ac7b22101`.
Portanto GREEN não resulta de alteração de testes depois de escrever produto.

A validação explícita B + V5 + norma congelada resolve a inconsistência do
par P1319 sem corrigir ferramenta externa. A limitação da ferramenta continua
registrada. O autor comunicou uma microcorreção posterior para explicitar
Pure/File válido no match e retirar warning V16; novos hashes/gates r2 ainda
serão julgados antes do aceite final. A/B não havia recebido binário.
