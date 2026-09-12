# Prompt L0 — contrato externo do snapshot de `watch`
Hash do Código: 771a5a67

**Camada:** L3 — teste de integração
**Ficheiro alvo exclusivo:** `03_infra/tests/p1295_watch_contract.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Propriedade

Este prompt possui exclusivamente o integration test acima. O código produtivo
de filesystem pertence a `shell/watch.md`. O teste observa a API pública sem
fixar campos, layout, algoritmo de fingerprint ou representação da coleção.

## Medição anterior à decisão

`snapshot` captura o estado antes do estímulo e `wait_for_change_since` recebe
o token capturado. Isso distingue a API contratada de uma espera que recapture
o baseline depois da alteração. A API legada `wait_for_change` permanece.

## Contrato funcional

Cada caso usa diretório temporário exclusivo e limpeza RAII. O consumer deve:

1. detectar troca de conteúdo pelo mesmo comprimento após `snapshot`;
2. detectar criação posterior de path inicialmente ausente;
3. detectar remoção posterior de path inicialmente presente;
4. fixar por type-check `snapshot`, `wait_for_change_since` e
   `wait_for_change`;
5. em Linux, provar que a API legada retorna para um pseudo-ficheiro público
   cujo conteúdo muda entre leituras.

As transições terminam antes da thread de espera começar. `recv_timeout`
limita falha; não é sinal de prontidão. Não usar sleep, retry, carga artificial
ou segunda alteração corretiva.

## Superfície pública

Com a toolchain pinada no consumer, rustdoc JSON deve provar:

- `WatchSnapshot` público com estado privado não vazio;
- nenhum campo, item inherent ou trait direto público;
- somente `snapshot` e `wait_for_change_since` entre funções públicas cujas
  assinaturas referenciam o token.

A enumeração aceita representação named ou tuple e não escolhe nomes ou tipos
de campos. Versão, schema, JSON ou identidade ausente fazem o teste falhar.

## Aceitação

Os cinco observáveis, assinaturas e inventário passam. O contrato não declara
equivalência geral de `watch` nem substitui os testes de processo da CLI.
