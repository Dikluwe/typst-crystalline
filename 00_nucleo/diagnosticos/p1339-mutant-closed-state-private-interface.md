# P1339 — suplemento de interface privada e inventário declarativo

Prospectivo e anterior ao candidato. Complementa sem sobrescrever `p1339-mutant-closed-state-interface.md` SHA-256 `7ff05129bd9213ffbe1a9c72b403262559834c782bb9f10928ea7d20bfd02f8d`. Autoridade de tradução: `p1339-closed-harness-authority.json` SHA-256 `08d0be7f19894c111130d0c791df6804cd1e7d89d2b8d6f418702be88af2125d`. Nenhuma expectativa ou relação de observações é decidida aqui.

## Inventário de dados, não prova de cobertura

`p1339-mutant-closed-state-inventory.json`, SHA-256 `d0ab8a2a2297a8f8fcff36ac372a8b418368b814c92cf25184fbe17f7efd101f`, contém as declarações literais sem comentários/corpos para Value, IntrospectedContent, Content, Args, ArgOccurrence, Selector, SourceDiagnostic, Severity, Tracepoint, State, Counter, CounterKey, Func e seus carriers, Module e ModuleInner, mais dependências nominais encontradas. Cada declaração guarda fonte, linhas e hash; o recibo identifica HEAD e UTC.

A extração mediu 38 variantes diretas de Value e 98 de Content. Há 213 declarações selecionadas, 16 tokens de tipo não expandidos e três nomes ambíguos (Engine, Selector, NodeKind). A seleção transita por nomes, **não** por resolução Rust: ela retém candidatos ambíguos para auditoria, não declara que todos os tipos são relevantes nem que as folhas externas estão cobertas. É insumo do oráculo/verificador para mapear cada variante e campo para teste concreto ou testemunha estrutural normativa; não converte uma amostra de tipos em exaustividade.

O Selector do valor público é `entities/selector.rs`; `entities/show.rs::Selector` é o carrier distinto das regras show. Ambos estão registrados, sem unificação por nome. O arquivo traz também State/Counter completos, sem presumir que sejam primitivos. Tipos externos/traits como World, FontMetrics, PluginHost e DynElement ficam explicitamente não expandidos. Nenhum deles recebe cobertura implícita.

## Carrier opaco real disponível

Não existe variante ativa `Value::Dynamic` nem `Value::Dyn` no baseline. O carrier real é:

```rust
Value::Content(Content::Dynamic(Arc<dyn DynElement>))
pub fn Content::dynamic<E: DynElement>(elem: E) -> Content;
```

A fixture Rust existente `crate::entities::elements::test_callout::CalloutElem` é disponível somente sob teste (mod.rs:128-129) e oferece:

```rust
pub struct CalloutElem {
    pub body: Content,
    pub title: EcoString,
    pub tone: EcoString,
}
impl CalloutElem {
    pub fn new(body: Content, title: impl Into<EcoString>,
        tone: impl Into<EcoString>) -> Self;
}
```

Uma operação mecânica `dynamic_callout {body_binding, title, tone, output_binding}` pode chamar esse constructor e `Content::dynamic`, guardando `Value::Content` no binding nomeado. `clone_binding` clona o valor já construído; executar novamente `dynamic_callout` constrói outra instância. A escolha entre retenção, nova construção, conteúdos e relação esperada pertence ao autor do oráculo. Não implementar DynElement à mão, não usar enum fictício, não usar `dyn_eq`/Debug como oráculo de estabilidade.

Fonte de assinaturas: `content.rs:1175,1566`, `elements/dynamic.rs:44`, `elements/test_callout.rs:27-38`, `elements/mod.rs:128-129`. Os hashes desses arquivos ficam no inventário ou no recibo final do adaptador; nenhum corpo de método é entregue como expectativa.

## Slots privados que precisam de ligação real posterior

As três APIs públicas não expõem o discriminante privado de relação nem todo o ciclo de retenção. Em particular, `Ok(false)` **não distingue** Different de Unproven. F06–F08 não podem ser classificados como executados apenas por chamar essas APIs.

Protocolo simbólico a ser congelado com as fixtures e autorizado no contrato final:

1. `compare_observation_value(left_binding, right_binding)` seleciona a operação privada **real usada pelo validador** e devolve seu discriminante real Same/Different/Unproven. Um vínculo test-only pode converter nomes/variantes para o formato de saída, mas não calcular a relação, substituir por igualdade pública ou comparar valores renderizados.
2. `retention_probe` executa o caminho real de avaliação/validação/retomada indicado pela fixture e captura tokens de identidade reais antes/depois, separadamente do valor observado. Os tokens só observam retenção; não escolhem estabilidade nem acionam reavaliação.
3. `lifecycle_probe` observa tentativas, dependências, retenção/descarte e sinks no caminho produtivo real solicitado. O adaptador não implementa um mini-orquestrador alternativo que antecipe o resultado.

Os nomes e assinaturas Rust privados ainda não existem. Portanto **não há ligação compilável congelada desses slots neste documento**, nem execução preseal. A exceção de ligação tardia precisa estar explícita no contrato: somente um shim owner-local `#[cfg(test)]`, com mapeamento de cada slot ao owner/call graph reais e auditoria independente, sem alterar fixture/predicado ou introduzir API produtiva. Se a assinatura efetiva exigir lógica semântica nova no adaptador, ou a operação real não puder ser localizada, o vínculo não é mecânico e o gate fica aberto.

A forma concreta do harness, seus argumentos e predicados só serão gerados após os fixtures canônicos independentes com hashes. O verificador deve auditar tanto a fronteira de capacidade quanto a exaustividade: estes slots são plano de integração, não aliases que por si só provem cobertura.
