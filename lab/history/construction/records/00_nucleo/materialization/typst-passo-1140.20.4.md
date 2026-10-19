# Passo 1140.20.4 — Supplement e referências de página

**Estado:** escrito — não executado  
**Data:** 2026-08-24  
**Continua:** P1140.20.3  
**Prepara:** P1140.21  
**Gate:** ADR-0127 obrigatório antes de código

## Objetivo

Implementar `page.supplement` como metadado lógico de página, incluindo
`auto`, `none`, content, armazenamento, introspecção e consumo por
`ref(..., form: "page")`, sem confundi-lo com header/footer.

## Fase A — L0 e gate

1. Revalidar `page.rs:313-324`, `pages/run.rs:144-148,230-244`,
   `document.rs:88-121`, `introspection/location.rs:263-289` e
   `model/reference.rs:249-257,334-355`.
2. Auditar `Page`, `PageStore`, introspector, location e referências do
   cristalino; localizar o owner mínimo sem duplicar metadado.
3. Especificar auto como nome local normativo de page, none como conteúdo
   vazio e content preservado; fixar espaçamento/morfologia da referência.
4. Especificar fixpoint, isolamento de page-run e páginas multipágina.
5. Atualizar L0s realmente tocados, marcar que P1140.21 só adiciona binding e
   diagnósticos, resselo e **parar**.

## Fase B — RED→GREEN, após confirmação

- RED para auto/none/content e restauração;
- RED para referência à página de heading/label sob numeração configurada;
- RED para função de numbering receber um argumento em referência;
- RED para conteúdo vazio, páginas múltiplas e convergência;
- implementar transporte e introspecção atomizados, incluindo helpers L3
  somente quando houver I/O ou travessia de representação externa.

## Aceitação

Supplement correto por localização e page-run; referência preserva conteúdo e
numeração lógica; nenhum acoplamento a marginais visuais; nenhuma lacuna das 15
propriedades permanece antes de P1140.21; testes, build, lint e diff check.
