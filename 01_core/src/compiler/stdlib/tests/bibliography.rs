//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/bibliography.md
//! @prompt-hash 3bdd851d
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn native_bibliography_default_vazia() {
    // P479: bibliography() sem args produz título default Content::heading(1, "Bibliography").
    null_ctx!(ctx);
    let r =
        native_bibliography(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        assert!(e.entries.is_empty());
        // P479: título default é heading nível 1 "Bibliography" (não None).
        assert!(e.title.is_some(), "P479: título default deve ser Some");
        let title_text = e.title.as_ref().unwrap().plain_text();
        assert_eq!(title_text, "Bibliography", "P479: texto do título default");
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_com_entries_posicional() {
    null_ctx!(ctx);
    let entries_arr = Value::Array(vec![make_bib_dict(
        "smith2024",
        "Smith, J.",
        "On Crystal Math",
        2024,
    )]);
    let args = p(vec![entries_arr]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        assert_eq!(e.entries.len(), 1);
        assert_eq!(e.entries[0].key, "smith2024");
        assert_eq!(e.entries[0].author, "Smith, J.");
        assert_eq!(e.entries[0].year, 2024);
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_com_title() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Array(vec![])]);
    args.named.insert("title".into(), Value::Str("Referências".into()));
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        assert_eq!(
            e.title.as_ref().map(|t| t.plain_text()).as_deref(),
            Some("Referências")
        );
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn p479_native_bibliography_default_titulo_e_heading() {
    null_ctx!(ctx);
    let r =
        native_bibliography(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let title = e.title.as_ref().expect("P479: default title deve ser Some");
        assert!(
            matches!(title, Content::Heading(_)),
            "P479: default title deve ser Content::Heading, obtido {:?}",
            title
        );
        assert_eq!(title.plain_text(), "Bibliography");
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn p479_native_bibliography_title_none_suprime_titulo() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("title".into(), Value::None);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        assert!(e.title.is_none(), "P479: title: none deve suprimir o título");
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn p479_native_bibliography_title_explicito_preservado() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("title".into(), Value::Str("Referências".into()));
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let title = e.title.as_ref().expect("P479: título explícito deve ser Some");
        assert_eq!(title.plain_text(), "Referências");
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_dict_sem_field_obrigatorio_rejeitado() {
    // P159A: dict sem 'year' (ou outro field obrigatório) → erro hard.
    null_ctx!(ctx);
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str("k".into()));
    d.insert("author".into(), Value::Str("A".into()));
    d.insert("title".into(), Value::Str("T".into()));
    // year ausente intencionalmente.
    let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "dict sem 'year' deve retornar Err");
}

#[test]
fn native_bibliography_year_negativo_rejeitado() {
    null_ctx!(ctx);
    let args = p(vec![Value::Array(vec![make_bib_dict("k", "A", "T", -5)])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "year negativo deve retornar Err");
}

#[test]
fn native_bibliography_named_arg_desconhecido_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Array(vec![])]);
    args.named.insert("full".into(), Value::Bool(true));
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "full (scope-out per ADR-0054 graded) deve retornar Err");
}

#[test]
fn native_bibliography_style_ieee_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Array(vec![])]);
    args.named.insert("style".into(), Value::Str("ieee".into()));
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Bibliography(_))));
    // P429: style resolvido é registado no EvalContext, não no elemento.
    assert_eq!(ctx.bibliography_styles.len(), 1);
    let style = ctx.bibliography_styles.values().next().unwrap();
    assert!(style.info.title.value.contains("IEEE"));
}

#[test]
fn native_bibliography_path_bib_carrega_entries() {
    null_ctx!(ctx);
    let mut world = null_world();
    world.files.insert(
        "refs.bib".into(),
        std::sync::Arc::new(
            b"@article{smith2024, author = {Smith, J.}, title = {On Crystal Math}, year = {2024}}"
                .to_vec(),
        ),
    );
    let args = p(vec![Value::Str("refs.bib".into())]);
    let r = native_bibliography(&mut ctx, &args, &world, test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        assert_eq!(e.entries.len(), 1);
        assert_eq!(e.entries[0].key, "smith2024");
        assert_eq!(e.path.as_deref(), Some("/refs.bib"));
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_path_yaml_carrega_entries() {
    null_ctx!(ctx);
    let mut world = null_world();
    world.files.insert(
        "refs.yaml".into(),
        std::sync::Arc::new(
            b"smith2024:\n  type: article\n  author: Smith, J.\n  title: On Crystal Math\n  year: 2024"
                .to_vec(),
        ),
    );
    let args = p(vec![Value::Str("refs.yaml".into())]);
    let r = native_bibliography(&mut ctx, &args, &world, test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        assert_eq!(e.entries.len(), 1);
        assert_eq!(e.entries[0].key, "smith2024");
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_path_nao_encontrado_erro() {
    null_ctx!(ctx);
    let args = p(vec![Value::Str("inexistente.bib".into())]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "path inexistente deve retornar erro");
    let msg = r.unwrap_err()[0].message.clone();
    assert!(msg.contains("não encontrado") || msg.contains("not found"), "{msg}");
}

#[test]
fn native_bibliography_path_e_style_preservam() {
    null_ctx!(ctx);
    let mut world = null_world();
    world.files.insert(
        "refs.bib".into(),
        std::sync::Arc::new(
            b"@article{smith2024, author = {Smith, J.}, title = {T}, year = {2024}}"
                .to_vec(),
        ),
    );
    let mut args = p(vec![Value::Str("refs.bib".into())]);
    args.named.insert("style".into(), Value::Str("ieee".into()));
    args.named.insert("locale".into(), Value::Str("en-US".into()));
    let r = native_bibliography(&mut ctx, &args, &world, test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        assert_eq!(e.path.as_deref(), Some("/refs.bib"));
        assert_eq!(e.style.as_deref(), Some("ieee"));
        assert_eq!(e.locale.as_deref(), Some("en-US"));
        // P429: style resolvido viaja no EvalContext (e depois no Module/BibStore),
        // indexado pela chave determinística do elemento.
        let key = e.style_key();
        assert!(ctx.bibliography_styles.contains_key(&key));
        let style = ctx.bibliography_styles.get(&key).unwrap();
        assert!(style.info.title.value.contains("IEEE"));
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_cite_so_key_posicional() {
    null_ctx!(ctx);
    let r = native_cite(
        &mut ctx,
        &p(vec![Value::Str("smith2024".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Cite(e)) = r {
        assert_eq!(e.key, "smith2024".to_string());
        assert!(e.supplement.is_none());
        assert!(e.form.is_none());
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn native_cite_com_supplement() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("smith2024".into())]);
    args.named.insert("supplement".into(), Value::Str("p. 42".into()));
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Cite(e)) = r {
        assert_eq!(
            e.supplement.as_ref().map(|s| s.plain_text()).as_deref(),
            Some("p. 42")
        );
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn native_cite_sem_key_rejeitado() {
    null_ctx!(ctx);
    let r = native_cite(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "cite() sem key deve retornar Err");
}

#[test]
fn native_cite_key_vazia_rejeitada() {
    null_ctx!(ctx);
    let r = native_cite(
        &mut ctx,
        &p(vec![Value::Str("".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "cite() key vazia deve retornar Err");
}

#[test]
fn native_cite_named_arg_desconhecido_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("k".into())]);
    args.named.insert("style".into(), Value::Str("apa".into()));
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "style (scope-out per ADR-0054 graded) deve retornar Err");
}

#[test]
fn native_cite_form_normal_parse() {
    use crate::entities::citation_form::CitationForm;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("k".into())]);
    args.named.insert("form".into(), Value::Str("normal".into()));
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Cite(e)) = r {
        assert_eq!(e.form, Some(CitationForm::Normal));
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn native_cite_form_prose_parse() {
    use crate::entities::citation_form::CitationForm;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("k".into())]);
    args.named.insert("form".into(), Value::Str("prose".into()));
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Cite(e)) = r {
        assert_eq!(e.form, Some(CitationForm::Prose));
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn native_cite_form_author_parse() {
    use crate::entities::citation_form::CitationForm;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("k".into())]);
    args.named.insert("form".into(), Value::Str("author".into()));
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Cite(e)) = r {
        assert_eq!(e.form, Some(CitationForm::Author));
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn native_cite_form_year_parse() {
    use crate::entities::citation_form::CitationForm;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("k".into())]);
    args.named.insert("form".into(), Value::Str("year".into()));
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Cite(e)) = r {
        assert_eq!(e.form, Some(CitationForm::Year));
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn native_cite_form_auto_devolve_none() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("k".into())]);
    args.named.insert("form".into(), Value::Auto);
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Cite(e)) = r {
        assert!(
            e.form.is_none(),
            "form=auto deve produzir None (resolvido a Normal default em layout)"
        );
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn native_cite_form_invalido_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("k".into())]);
    args.named.insert("form".into(), Value::Str("xpto".into()));
    let r = native_cite(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "form inválido deve retornar Err");
    let msg = r.unwrap_err()[0].message.clone();
    assert!(
        msg.contains("normal")
            && msg.contains("prose")
            && msg.contains("author")
            && msg.contains("year"),
        "mensagem deve listar forms válidas: {}",
        msg
    );
}

#[test]
fn native_bibliography_parse_fields_opcionais_presentes() {
    null_ctx!(ctx);
    let entries_arr = Value::Array(vec![make_bib_dict_full(
        "smith2024",
        "Smith, J.",
        "On Crystal Math",
        2024,
        "12",
        "1-10",
        "Nature Communications",
        "ACM",
    )]);
    let args = p(vec![entries_arr]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let e = &e.entries[0];
        assert_eq!(e.volume.as_deref(), Some("12"));
        assert_eq!(e.pages.as_deref(), Some("1-10"));
        assert_eq!(e.journal.as_deref(), Some("Nature Communications"));
        assert_eq!(e.publisher.as_deref(), Some("ACM"));
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_parse_sem_fields_opcionais_regression_p159a() {
    // Regression: dict só com 4 obrigatórios produz entry com fields
    // opcionais None (output P159A inalterado).
    null_ctx!(ctx);
    let args = p(vec![Value::Array(vec![make_bib_dict(
        "smith2024",
        "Smith, J.",
        "On Crystal Math",
        2024,
    )])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let e = &e.entries[0];
        assert!(e.volume.is_none());
        assert!(e.pages.is_none());
        assert!(e.journal.is_none());
        assert!(e.publisher.is_none());
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_field_opcional_tipo_errado_rejeitado() {
    null_ctx!(ctx);
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str("k".into()));
    d.insert("author".into(), Value::Str("A".into()));
    d.insert("title".into(), Value::Str("T".into()));
    d.insert("year".into(), Value::Int(2024));
    // volume com tipo errado (Int em vez de Str).
    d.insert("volume".into(), Value::Int(42));
    let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "volume com tipo Int deve retornar Err");
    let msg = r.unwrap_err()[0].message.clone();
    assert!(msg.contains("volume"), "mensagem deve mencionar field 'volume': {}", msg);
}

#[test]
fn native_bibliography_parse_url_doi_presentes() {
    null_ctx!(ctx);
    let entries_arr = Value::Array(vec![make_bib_dict_with_url_doi(
        "smith2024",
        "Smith, J.",
        "On Crystal Math",
        2024,
        "https://example.com/paper",
        "10.1234/abc",
    )]);
    let args = p(vec![entries_arr]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let e = &e.entries[0];
        assert_eq!(e.url.as_deref(), Some("https://example.com/paper"));
        assert_eq!(e.doi.as_deref(), Some("10.1234/abc"));
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_parse_sem_url_doi_regression_p159d() {
    // Regression: dict só com 4 obrigatórios + 4 P159D opcionais
    // produz entry com url/doi None (output P159D inalterado).
    null_ctx!(ctx);
    let args = p(vec![Value::Array(vec![make_bib_dict(
        "smith2024",
        "Smith, J.",
        "On Crystal Math",
        2024,
    )])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let e = &e.entries[0];
        assert!(e.url.is_none());
        assert!(e.doi.is_none());
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_url_doi_tipo_errado_rejeitado() {
    null_ctx!(ctx);
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str("k".into()));
    d.insert("author".into(), Value::Str("A".into()));
    d.insert("title".into(), Value::Str("T".into()));
    d.insert("year".into(), Value::Int(2024));
    // doi com tipo errado (Int em vez de Str).
    d.insert("doi".into(), Value::Int(42));
    let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "doi com tipo Int deve retornar Err");
    let msg = r.unwrap_err()[0].message.clone();
    assert!(msg.contains("doi"), "mensagem deve mencionar field 'doi': {}", msg);
}

#[test]
fn native_bibliography_parse_p159g_fields_presentes() {
    null_ctx!(ctx);
    let entries_arr = Value::Array(vec![make_bib_dict_full_p159g(
        "smith2024",
        "Smith, J.",
        "On Crystal Math",
        2024,
        "Doe, A.",
        "Crystal Studies",
        "See also Smith 2023",
        "978-0-1234-5678-9",
        "New York",
        "ACM",
    )]);
    let args = p(vec![entries_arr]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let e = &e.entries[0];
        assert_eq!(e.editor.as_deref(), Some("Doe, A."));
        assert_eq!(e.series.as_deref(), Some("Crystal Studies"));
        assert_eq!(e.note.as_deref(), Some("See also Smith 2023"));
        assert_eq!(e.isbn.as_deref(), Some("978-0-1234-5678-9"));
        assert_eq!(e.location.as_deref(), Some("New York"));
        assert_eq!(e.organization.as_deref(), Some("ACM"));
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_parse_subset_p159g_fields() {
    // Parse com só editor + isbn presentes (3 fields P159G ausentes).
    null_ctx!(ctx);
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str("k".into()));
    d.insert("author".into(), Value::Str("A".into()));
    d.insert("title".into(), Value::Str("T".into()));
    d.insert("year".into(), Value::Int(2024));
    d.insert("editor".into(), Value::Str("Ed1".into()));
    d.insert("isbn".into(), Value::Str("978-0-1".into()));
    let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let e = &e.entries[0];
        assert_eq!(e.editor.as_deref(), Some("Ed1"));
        assert_eq!(e.isbn.as_deref(), Some("978-0-1"));
        // Outros P159G permanecem None.
        assert!(e.series.is_none());
        assert!(e.note.is_none());
        assert!(e.location.is_none());
        assert!(e.organization.is_none());
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_parse_sem_p159g_fields_regression_p159e() {
    // Regression: dict só com 4 obrigatórios + 4 P159D + 2 P159E
    // produz entry com 6 P159G fields None.
    null_ctx!(ctx);
    let args = p(vec![Value::Array(vec![make_bib_dict(
        "smith2024",
        "Smith, J.",
        "On Crystal Math",
        2024,
    )])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Bibliography(e)) = r {
        let e = &e.entries[0];
        assert!(e.editor.is_none());
        assert!(e.series.is_none());
        assert!(e.note.is_none());
        assert!(e.isbn.is_none());
        assert!(e.location.is_none());
        assert!(e.organization.is_none());
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn native_bibliography_isbn_tipo_errado_rejeitado() {
    null_ctx!(ctx);
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str("k".into()));
    d.insert("author".into(), Value::Str("A".into()));
    d.insert("title".into(), Value::Str("T".into()));
    d.insert("year".into(), Value::Int(2024));
    // isbn com tipo errado (Int em vez de Str).
    d.insert("isbn".into(), Value::Int(978));
    let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
    let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "isbn com tipo Int deve retornar Err");
    let msg = r.unwrap_err()[0].message.clone();
    assert!(msg.contains("isbn"), "mensagem deve mencionar field 'isbn': {}", msg);
}

#[test]
fn p472_back_refs_acumula_posicoes() {
    use crate::entities::bib_store::BibStore;
    let mut store = BibStore::empty();
    store.record_citation("a".to_string());
    store.record_citation("b".to_string());
    store.record_citation("a".to_string()); // segunda citação de "a"
    assert_eq!(store.back_refs_for_key("a"), vec![1, 3]);
    assert_eq!(store.back_refs_for_key("b"), vec![2]);
    assert!(store.back_refs_for_key("c").is_empty());
}

#[test]
fn p472_back_refs_nao_afecta_citation_order() {
    use crate::entities::bib_store::BibStore;
    let mut store = BibStore::empty();
    store.record_citation("b".to_string());
    store.record_citation("a".to_string());
    store.record_citation("b".to_string());
    // citation_order preserva primeira aparição
    assert_eq!(store.citation_order(), &["b", "a"]);
    // back_refs regista todas as chamadas
    assert_eq!(store.back_refs_for_key("b"), vec![1, 3]);
}

#[test]
fn p472_native_lof_sem_titulo() {
    null_ctx!(ctx);
    let result = super::structural::native_lof(
        &mut ctx,
        &p(vec![]),
        &null_world(),
        test_file_id(),
    );
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Content(c) => match c {
            crate::entities::content::Content::Outline(e) => {
                assert_eq!(
                    e.target,
                    crate::entities::elements::outline::OutlineTarget::Figures
                );
                assert!(e.title.is_none());
            }
            _ => panic!("esperado Outline"),
        },
        _ => panic!("esperado Content"),
    }
}

#[test]
fn p472_native_lot_sem_titulo() {
    null_ctx!(ctx);
    let result = super::structural::native_lot(
        &mut ctx,
        &p(vec![]),
        &null_world(),
        test_file_id(),
    );
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Content(c) => match c {
            crate::entities::content::Content::Outline(e) => {
                assert_eq!(
                    e.target,
                    crate::entities::elements::outline::OutlineTarget::Tables
                );
                assert!(e.title.is_none());
            }
            _ => panic!("esperado Outline"),
        },
        _ => panic!("esperado Content"),
    }
}

#[test]
fn p472_native_outline_target_figures() {
    null_ctx!(ctx);
    let args = pn(vec![], "target", Value::Str("figures".into()));
    let result =
        super::structural::native_outline(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Content(c) => match c {
            crate::entities::content::Content::Outline(e) => {
                assert_eq!(
                    e.target,
                    crate::entities::elements::outline::OutlineTarget::Figures
                );
            }
            _ => panic!("esperado Outline"),
        },
        _ => panic!("esperado Content"),
    }
}

#[test]
fn p472_native_outline_target_tables() {
    null_ctx!(ctx);
    let args = pn(vec![], "target", Value::Str("tables".into()));
    let result =
        super::structural::native_outline(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Content(c) => match c {
            crate::entities::content::Content::Outline(e) => {
                assert_eq!(
                    e.target,
                    crate::entities::elements::outline::OutlineTarget::Tables
                );
            }
            _ => panic!("esperado Outline"),
        },
        _ => panic!("esperado Content"),
    }
}

#[test]
fn p472_native_outline_target_invalido_erro() {
    null_ctx!(ctx);
    let args = pn(vec![], "target", Value::Str("unknown".into()));
    let result =
        super::structural::native_outline(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err());
}

#[test]
fn p472_outline_target_headings_default() {
    null_ctx!(ctx);
    let result = super::structural::native_outline(
        &mut ctx,
        &p(vec![]),
        &null_world(),
        test_file_id(),
    );
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Content(c) => match c {
            crate::entities::content::Content::Outline(e) => {
                assert_eq!(
                    e.target,
                    crate::entities::elements::outline::OutlineTarget::Headings
                );
            }
            _ => panic!("esperado Outline"),
        },
        _ => panic!("esperado Content"),
    }
}
