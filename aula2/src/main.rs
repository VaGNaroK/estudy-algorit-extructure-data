#[cfg(test)]
mod tests {
    // Traz as definições do escopo superior (como a estrutura MyVec) para o módulo de testes.
    use super::*;

    #[test]
    fn test_btsh_and_get() {
        // "let" é usado para declarar uma variável.
        // "let" indica que uma variável é imutável, seu valor pode ser alterado depois de definida.
        // "mut" indica que uma variável é mutável, ou seja, seu valor pode ser alterado depois de definida.
        // "MyVec::new()" chama a função associada a "new" da estrutura MyVec para criar uma nova instância do vetor.
        // Adiciona dois elementos.
        let mut vec = MyVec::new();

        vec.push(10);
        vec.push(20);

        // Verifica se os elementos foram inseridos corretamente.
        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(1), Some(&20));
        // Um índice que não existe deve retornar None.
        assert_eq!(vec.get(2), None);
    }
}