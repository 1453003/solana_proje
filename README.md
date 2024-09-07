Elbette! İşte daha ayrıntılı bir README dosyası örneği:

```markdown
# Event Ticketing System on Solana

## Açıklama

Bu proje, Solana blok zinciri üzerinde bir etkinlik biletleme sistemi oluşturmaktadır. Bu sistem sayesinde etkinlikler oluşturabilir, biletler mint edebilir ve bu biletleri başka kullanıcılara transfer edebilirsiniz. Proje, Solana ve Anchor kullanılarak geliştirilmiştir.

## Özellikler

- **Etkinlik Oluşturma**: Etkinlikler oluşturabilir, etkinlik adını ve tarihini belirleyebilirsiniz.
- **Bilet Mint Etme**: Etkinlikler için biletler mint edebilir ve biletleri belirli bir fiyat ile satabilirsiniz.
- **Bilet Transferi**: Biletleri başka kullanıcılara transfer edebilir ve transfer sırasında ödeme işlemleri gerçekleştirebilirsiniz.

## Gereksinimler

- **Rust**: Projeyi derlemek ve çalıştırmak için Rust 1.80.0 veya üzeri bir sürüm gereklidir.
- **Solana CLI**: Solana ağıyla etkileşimde bulunmak için Solana CLI kurulu olmalıdır.
- **Anchor CLI**: Anchor programlama çerçevesi için Anchor CLI'yi yüklemeniz gerekmektedir.

## Kurulum ve Kullanım

### 1. Rust ve Solana CLI Kurulumu

#### Rust Kurulumu

Rust'u kurmak için aşağıdaki komutu kullanabilirsiniz:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Kurulumdan sonra terminali kapatıp açarak Rust ortam değişkenlerini yükleyin.

#### Solana CLI Kurulumu

Solana CLI'yi kurmak için:
```sh
sh -c "$(curl -sSfL https://release.solana.com/v1.10.32/install)"
```

### 2. Anchor CLI Kurulumu

Anchor CLI'yi yüklemek için:
```sh
cargo install anchor-cli --locked
```

### 3. Projeyi Klonlama

Projeyi GitHub'dan klonlayın:
```sh
git clone https://github.com/username/event-ticketing-system.git
cd event-ticketing-system
```

### 4. Projeyi Derleme

Projeyi derlemek için:
```sh
anchor build
```

### 5. Test Aşaması

Testleri çalıştırmak için:
```sh
anchor test
```

## Kullanım

### Etkinlik Oluşturma

Etkinlik oluşturmak için `create_event` fonksiyonunu çağırabilirsiniz. Gerekli parametreler:
- `name`: Etkinlik adı
- `date`: Etkinlik tarihi (UNIX zaman damgası)

Fonksiyon çağrısı örneği:
```rust
pub fn create_event(ctx: Context<CreateEvent>, name: String, date: i64) -> Result<()>
```

### Bilet Mint Etme

Yeni bir bilet mint etmek için `mint_ticket` fonksiyonunu kullanabilirsiniz. Gerekli parametreler:
- `ticket_id`: Bilet ID'si
- `price`: Bilet fiyatı (lamports cinsinden)

Fonksiyon çağrısı örneği:
```rust
pub fn mint_ticket(ctx: Context<MintTicket>, ticket_id: u64, price: u64) -> Result<()>
```

### Bilet Transferi

Bir bileti başka bir kullanıcıya transfer etmek için `transfer_ticket` fonksiyonunu kullanabilirsiniz. Gerekli parametre:
- `new_owner`: Yeni bilet sahibi (Pubkey)

Fonksiyon çağrısı örneği:
```rust
pub fn transfer_ticket(ctx: Context<TransferTicket>, new_owner: Pubkey) -> Result<()>
```

## Hesap Yapıları

### Event

Etkinlikleri temsil eder:
- `name`: Etkinlik adı
- `date`: Etkinlik tarihi (UNIX zaman damgası)
- `organizer`: Etkinlik organizatörünün Pubkey'i

### Ticket

Biletleri temsil eder:
- `ticket_id`: Bilet ID'si
- `price`: Bilet fiyatı (lamports cinsinden)
- `owner`: Bilet sahibinin Pubkey'i

## Katkıda Bulunanlar

- [Rojin Orhan](https://github.com/1453003) - Proje geliştiricisi





