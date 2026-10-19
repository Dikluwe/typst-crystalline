from pathlib import Path
import json,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/clippy-expressions-2026-10-16');b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['all_source_hashes'].items())
decisions={
'export/builder/fonts.rs':'Substituir o match de compress_zlib com único braço útil por if let Ok(compressed), mantendo cutoff 256 KiB, construção da stream, retorno no sucesso e fallback cru no erro. Não mudar política de compressão ou dados.',
'export/builder/navigation.rs':'Expressar somente a opção do índice retido com pic.new_index(page_idx).map, preservando as condições anteriores: alvo nunca existente usa último retido, alvo válido excluído fica None. Não produzir heading nem alterar stack quando excluído.',
'export/html/pretty.rs':"Usar std::iter::repeat_n(' ', depth * 2) na mesma inserção de indentação. Preservar multiplicação, ordem, bytes e todos os contextos protegidos.",
'export/stream/text.rs':'No observador rendered_positions de testes, selecionar o último Tc com next_back no iterador duplo de linhas filtradas. O filtro strip_suffix é puro; conservar parse, fallback zero, cálculo independente de posições e todos os esperados TJ. Não alterar o emissor.',
'export/tests/p1486_pdf_page_labels_tests.rs':'Compilar as duas Regex constantes de St e S uma vez por invocação do observador, antes do loop de entries. Manter padrões idênticos, capturas, defaults 1/None, interpretação de prefixos e todos os controles por página. Sem estado global ou regex compartilhada entre invocações.',
'fonts/coverage.rs':'Somente no preparador de fontes sintéticas de teste, substituir len % 4 != 0 por !len.is_multiple_of(4). Preservar padding de zeros, offset/length e conteúdo cmap. Não alterar extração/união produtiva nem os arrays unitários de Range neste lote.',
'pipeline/paged.rs':'Trocar shared_metrics.map(Clone::clone) por shared_metrics.cloned(), mantendo unwrap_or_else preguiçoso e o contador de métricas sem cache no mesmo ramo. Mesma instância compartilhada, ciclo, World, callback budget e comportamento de ausência.',
'pipeline/bundle.rs':'Reutilizar a Box existente do body ao aplicar o envelope Styled: atribuir ao conteúdo via **body o mesmo Content::styled(body.clone(), styles.clone()). O clone do corpo deve ser avaliado antes da substituição. Preservar envelope, ordem, estilos, origem e causalidade; endereço/alocação do Box externo é mecânica privada.',
'pipeline/tests.rs':'No contador de pontos de destino, incorporar LinkTarget::Destination(_) ao pattern FrameItem::Link em vez do guard matches. Preservar recursão pelos demais wrappers, contagem, fixtures e asserções.',
'world/font_session.rs':'No teste de aquisição preguiçosa, usar !world.book().is_empty() em vez de len > 0, mantendo ordem das consultas/assertions e verificação de cache. Não alterar new(), Default, aquisição ou política de sessão.',
'world/tests.rs':'No teste do catálogo de fontes do projeto, usar !world.font_catalog().0.is_empty() em vez de len >= 1. Conservar o acesso, fontes válidas e o requisito de catálogo não vazio.',
'integration_tests.rs':'Elidir o lifetime único de project_item mantendo o mesmo empréstimo de entrada/retorno e projeções geométricas. Nos controles de Glyph, incorporar x/𝑥 e y/𝑦 ao pattern em vez de guards. Usar arrays RGBA .repeat(4) nos mesmos fixtures de PNG, preservando bytes/tamanhos e expectativas. A correção do observador parse_pdf_objects é especificada abaixo, em vez de apenas achatar seu else.',
'watch.rs':'Achatar somente else { if Bundle ... } para else if Bundle no descarte após falha. Preservar abandon_directory versus abandon, staging, publicação apenas no sucesso, retorno e estado anterior disponível.',
'crystalline_lint.rs':'Nos helpers privados write_file e setup_project_dirs, receber &Path em vez de &PathBuf; manter PathBuf no retorno temp_project e adicionar o import Path. Os chamadores continuam os mesmos, com coercion por empréstimo, sem cópias ou mudança de limpeza/processos/V14.',
'p1293_contract.rs':'No controle viewBox, iterar expected.iter().enumerate() sobre o array de quatro coordenadas, conservando índices, comparação com summary.0[index], tolerância e mensagens. Não usar zip que trunque silenciosamente uma observação curta. Em push_dom_node, somente stack passa de &mut Vec<DomNode> a &mut [DomNode], pois só last_mut é usado; roots permanece Vec para push. Todos os chamadores privados e DOM/children ficam iguais.',
'p1462_format_svg.rs':'Usar &source.as_bytes()[start..end] no observador XML: start/end vêm de find de tags ASCII no str UTF-8 e são fronteiras válidas. Preservar limites e erros existentes. Usar ? somente na extração de artifact, depois de require_success, conservando registro de falha e None.'}
records={}
for owner in sorted({c['owner'] for c in b['cases']}):
 cs=[c for c in b['cases'] if c['owner']==owner];p=cs[0]['path'];key=p.removeprefix('03_infra/src/')
 if p.startswith('04_wiring/'):key=p.split('/')[-1]
 d=decisions[key];evidence='; '.join(f"{c['path']}:{c['line']} ({c['rule']})" for c in cs)
 extra=''
 if p.endswith('/integration_tests.rs'):
  extra='''\n### Observador PDF — correção funcional interna dos testes\n\nMedição: integration_tests.rs:1273 reconhece qualquer `<` como hex antes de :1275 testar `<<`; o incremento de depth desse segundo ramo é inalcançável. :1285-1286 insere o corpo até i-2 mesmo se o loop terminou por EOF com depth positivo. Isso pode omitir campos após dicionário aninhado e aceitar observação truncada. É defeito do observador local, não evidência de defeito no PDF produzido.\n\nO helper continua privado e limitado a objetos iniciados por `id 0 obj <<`. Reconhecer o delimitador `<<` antes do `<` de hex, manter depth simétrico com `>>`, proteger os mesmos literais e escapes e exigir fechamento completo antes de inserir o corpo. Truncamento deve falhar explicitamente, nunca conceder aceitação a corpo parcial. Não criar um parser PDF geral nem alterar observação de streams, catálogo, outlines ou expectativas produtivas. Duas regressões independentes: corpo conhecido com dicionários aninhados, literal com `>>`/escape e campos após o filho deve permanecer inteiro; objeto sem fechamento deve falhar. Controles plano e objetos subsequentes devem permanecer observáveis. Confirmar ambas RED no helper recebido antes de implementá-las, depois GREEN e revalidar as regressões reais de outlines.\n'''
 text=f'''\n\n## Expressões locais verificadas — Clippy 2026-10-16\n\n### Medição anterior à decisão\n\n{evidence}. O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.\n\n### Responsabilidade exclusiva e aceitação\n\n{d}\n{extra}\nFluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.\n'''
 assert Path(owner).read_text()==b['files'][owner]['text'];Path(owner).write_text(b['files'][owner]['text']+text);records[owner]=sha(owner)
(out/'l0-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owners=records),indent=2)+'\n')
# Add only the independent observer regressions before changing the helper.
p=Path('03_infra/src/integration_tests.rs');s=p.read_text();anchor='    /// Devolve os valores de `/Count` presentes apenas na árvore de bookmarks.\n';assert s.count(anchor)==1
new=r'''    #[test]
    fn pdf_object_observer_preserves_fields_after_nested_dictionaries() {
        let pdf = br"1 0 obj << /Before 1 /Nested << /Hex <3e3e> /Text (literal >> and \) escaped) /Empty <<>> >> /After 7 >> endobj
2 0 obj << /Second true >> endobj";
        let objects = parse_pdf_objects(pdf);
        assert_eq!(objects.len(), 2);
        assert_eq!(
            objects[&1],
            r" /Before 1 /Nested << /Hex <3e3e> /Text (literal >> and \) escaped) /Empty <<>> >> /After 7 "
        );
        assert_eq!(objects[&2], " /Second true ");
    }

    #[test]
    #[should_panic(expected = "unterminated PDF dictionary observation")]
    fn pdf_object_observer_rejects_truncated_dictionary() {
        parse_pdf_objects(b"1 0 obj << /Count 3 /Nested << /Value 1 >>");
    }

'''
s=s.replace(anchor,new+anchor);p.write_text(s)
(out/'observer-tests-before-implementation.rs.txt').write_text(new)
print('L0 updated before tests:',len(records))
