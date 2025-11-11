# 🔧 Correção de Estrutura do Banco de Dados

## ❌ Problema Identificado

O código do servidor em Rust foi desenvolvido esperando as seguintes colunas no banco de dados:
- `transaction_type` (na tabela `transactions`)
- `category_type` (na tabela `categories`)

No entanto, a estrutura atual do banco de dados utiliza a coluna genérica `type` em ambas as tabelas. Essa incompatibilidade de nomes impede o correto mapeamento dos dados pelo ORM do Rust.

## ✅ Solução Aplicada: Colunas Virtuais

A solução mais segura e recomendada é a adição de **colunas virtuais** (Generated Columns) que atuam como aliases. Essa abordagem permite que o código Rust utilize os nomes de coluna esperados (`transaction_type` e `category_type`) sem a necessidade de alterar ou renomear a coluna original `type`, garantindo a compatibilidade com qualquer código legado.

### 1. Execução do Script de Correção

Para aplicar a correção, utilize o script `fix_columns.sql`.

**Opção A: Via Interface Gráfica (MySQL Workbench)**

1.  Conecte-se ao servidor MySQL e selecione o banco de dados `alpha_bank`.
2.  Vá em `File` → `Open SQL Script`.
3.  Selecione o arquivo `fix_columns.sql`.
4.  Execute o script.

**Opção B: Via Linha de Comando**

```bash
mysql -u root -p alpha_bank < fix_columns.sql
```

### 2. Verificação da Estrutura

Após a execução, é possível verificar a nova estrutura das tabelas:

```sql
-- Ver estrutura da tabela categories
DESCRIBE categories;

-- Ver estrutura da tabela transactions
DESCRIBE transactions;

-- Testar se os aliases funcionam
SELECT id, name, type, category_type FROM categories LIMIT 3;
SELECT id, description, type, transaction_type FROM transactions LIMIT 3;
```

As colunas `category_type` e `transaction_type` devem estar presentes, refletindo o mesmo valor da coluna `type`.

### 3. Sobre Colunas Virtuais

Colunas definidas como **GENERATED ALWAYS AS (expressão) VIRTUAL** possuem as seguintes vantagens:

*   **Eficiência de Espaço:** Não consomem espaço em disco, pois são calculadas em tempo real.
*   **Sincronização:** Estão sempre sincronizadas com a coluna de origem.
*   **Flexibilidade:** Podem ser indexadas e são compatíveis com versões recentes do MySQL (5.7+).

### 4. Conclusão

Após a aplicação do script, o servidor Rust deve ser reiniciado para reconhecer as novas colunas virtuais e permitir o correto funcionamento dos endpoints.

```bash
cargo run
```

---

## 🔄 Alternativa (Renomear Colunas)

Caso seja estritamente necessário renomear as colunas permanentemente, o seguinte comando pode ser utilizado. **Atenção:** Esta ação pode quebrar queries antigas e não é a abordagem recomendada.

```sql
-- ATENÇÃO: Isso renomeará permanentemente as colunas!

ALTER TABLE categories 
CHANGE COLUMN type category_type VARCHAR(20) NOT NULL 
CHECK (category_type IN ('income', 'expense', 'both'));

ALTER TABLE transactions 
CHANGE COLUMN type transaction_type VARCHAR(20) NOT NULL 
CHECK (transaction_type IN ('income', 'expense'));
```
