# 🔧 Correção do Banco de Dados

## ❌ Problema

O código Rust espera as colunas:
- `transaction_type` (na tabela transactions)
- `category_type` (na tabela categories)

Mas o banco de dados tem:
- `type` (em ambas as tabelas)

## ✅ Solução

Adicionar **colunas virtuais** (GENERATED) que funcionam como aliases. Isso mantém a coluna original `type` e cria as colunas que o Rust espera.

---

## 📋 Passo a Passo

### 1. Abrir MySQL Workbench

1. Conecte ao servidor MySQL
2. Selecione o banco `alpha_bank`

### 2. Executar Script de Correção

**Opção A: Via Interface**

1. File → Open SQL Script
2. Selecione o arquivo `fix_columns.sql`
3. Clique no raio ⚡ (Execute)

**Opção B: Via Command Line**

```bash
mysql -u root -p alpha_bank < fix_columns.sql
```

### 3. Verificar

Execute no MySQL Workbench:

```sql
-- Ver estrutura da tabela categories
DESCRIBE categories;

-- Ver estrutura da tabela transactions
DESCRIBE transactions;

-- Testar se os aliases funcionam
SELECT id, name, type, category_type FROM categories LIMIT 3;
SELECT id, description, type, transaction_type FROM transactions LIMIT 3;
```

Você deve ver as novas colunas `category_type` e `transaction_type` com o mesmo valor de `type`.

---

## 🔍 O que são Colunas Virtuais?

Colunas **GENERATED ALWAYS AS (expressão) VIRTUAL** são:

✅ **Não ocupam espaço** em disco (calculadas em tempo real)  
✅ **Sempre sincronizadas** com a coluna original  
✅ **Podem ser indexadas** (se necessário)  
✅ **Compatíveis** com MySQL 5.7+  

**Exemplo:**
```sql
-- Quando você insere:
INSERT INTO categories (name, type) VALUES ('Salário', 'income');

-- Automaticamente category_type = 'income'
-- Sem precisar inserir manualmente!
```

---

## 🚀 Após a Correção

1. **Reinicie o servidor Rust:**
   ```bash
   cargo run
   ```

2. **Teste os endpoints:**
   ```powershell
   # Listar categorias (deve funcionar agora)
   Invoke-WebRequest -Uri http://localhost:8080/api/categories `
       -Headers @{"Authorization" = "Bearer SEU_TOKEN"}
   ```

3. **Criar transação:**
   ```powershell
   $body = @{
       description = "Teste"
       amount = 100.00
       type = "income"
       category_id = $null
   } | ConvertTo-Json

   Invoke-WebRequest -Uri http://localhost:8080/api/transactions `
       -Method POST `
       -Headers @{"Authorization" = "Bearer SEU_TOKEN"} `
       -ContentType "application/json" `
       -Body $body
   ```

---

## 🔄 Alternativa (Se Preferir)

Se você quiser **renomear as colunas** ao invés de criar aliases:

```sql
-- ATENÇÃO: Isso vai renomear permanentemente as colunas!

ALTER TABLE categories 
CHANGE COLUMN type category_type VARCHAR(20) NOT NULL 
CHECK (category_type IN ('income', 'expense', 'both'));

ALTER TABLE transactions 
CHANGE COLUMN type transaction_type VARCHAR(20) NOT NULL 
CHECK (transaction_type IN ('income', 'expense'));
```

**Mas recomendo usar as colunas virtuais** (primeira opção) pois:
- Mantém compatibilidade com código existente
- Não quebra queries antigas
- Mais seguro

---

## ✅ Pronto!

Após executar o script `fix_columns.sql`, os erros devem desaparecer! 🎉

