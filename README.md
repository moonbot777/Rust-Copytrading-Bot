# Raypump Copy Trading Bot

A high-performance Rust-based copy trading bot for Solana that monitors Pump.fun DEX and automatically copies trades from target wallets. The bot uses Yellowstone gRPC for real-time transaction monitoring and supports both Jito and Nozomi MEV protection services.

## 🚀 Features

- **Real-time Monitoring**: Uses Yellowstone gRPC to monitor Pump.fun transactions in real-time
- **Copy Trading**: Automatically copies buy/sell transactions from target wallets
- **MEV Protection**: Supports both Jito and Nozomi for transaction protection
- **Configurable Parameters**: 
  - Take profit and stop loss percentages
  - Market cap thresholds
  - Buy/sell percentage amounts
  - Slippage tolerance
- **Target Wallet Management**: Load target wallets from `targetlist.txt`
- **Auto Liquidation**: Automatically sells tokens when take profit or stop loss is reached
- **Market Cap Filtering**: Only buys tokens that meet minimum market cap requirements

## 📋 Prerequisites

- Rust 1.70+ 
- Node.js and npm
- PM2 (for process management)
- Solana CLI tools
- Valid Solana wallet with SOL balance

## 🛠️ Installation

1. **Clone the repository**
```bash
git clone https://github.com/moonbot777/Copy-trading-bot-rust.git
cd Copy-trading-bot-rust
```

2. **Install dependencies**
```bash
make install
```

3. **Build the project**
```bash
make build
```

## ⚙️ Configuration

Create a `.env` file in the project root with the following variables:

```env
# RPC Configuration
RPC_HTTPS=https://your-rpc-endpoint.com
YELLOWSTONE_GRPC_HTTP=https://your-yellowstone-grpc-endpoint.com
YELLOWSTONE_GRPC_TOKEN=your-yellowstone-token

# Wallet Configuration
PRIVATE_KEY=your-wallet-private-key

# Trading Parameters
BUY_TOKEN_PERCENTAGE=100.0
SELL_TOKEN_PERCENTAGE=100.0
SLIPPAGE=1
TP=3.0
SL=0.5
MC=0.0

# MEV Protection Services
NOZOMI_URL=https://your-nozomi-endpoint.com
NOZOMI_TIP_VALUE=0.001
JITO_BLOCK_ENGINE_URL=https://your-jito-endpoint.com
JITO_TIP_VALUE=0.001
```

### Configuration Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `BUY_TOKEN_PERCENTAGE` | Percentage of target's buy amount to copy | 100.0 |
| `SELL_TOKEN_PERCENTAGE` | Percentage of target's sell amount to copy | 100.0 |
| `SLIPPAGE` | Slippage tolerance in basis points (0-100) | 1 |
| `TP` | Take profit multiplier (e.g., 3.0 = 3x profit) | 3.0 |
| `SL` | Stop loss multiplier (e.g., 0.5 = 50% loss) | 0.5 |
| `MC` | Minimum market cap threshold in SOL | 0.0 |

## 📝 Target Wallet Setup

Create a `targetlist.txt` file in the project root with one wallet address per line:

```
wallet_address_1
wallet_address_2
wallet_address_3
```

The bot will monitor these wallets and copy their Pump.fun transactions.

## 🚀 Usage

### Basic Commands

```bash
# Build the project
make build

# Start with PM2
make pm2

# Start the bot
make start

# Stop the bot
make stop

# Run ping test
cargo run ping_test
```

### Advanced Usage

```bash
# Build for specific targets
make build-x86_64  # 64-bit Windows
make build-i686    # 32-bit Windows

# Clean build artifacts
make clean

# Package for distribution
make package
```

## 🔧 Architecture

### Core Components

- **Monitor Engine** (`src/engine/monitor.rs`): Real-time transaction monitoring and copy trading logic
- **Pump.fun DEX** (`src/dex/pump_fun.rs`): Integration with Pump.fun bonding curve protocol
- **Transaction Services** (`src/services/`): MEV protection services (Jito/Nozomi)
- **Configuration** (`src/common/config.rs`): Environment and trading parameter management

### Key Features

1. **Real-time Monitoring**: Uses Yellowstone gRPC to subscribe to Pump.fun transactions
2. **Smart Copy Trading**: Analyzes transaction data to determine buy/sell actions
3. **Risk Management**: Implements take profit, stop loss, and market cap filters
4. **MEV Protection**: Integrates with Jito and Nozomi for transaction protection
5. **Auto Liquidation**: Automatically sells tokens when conditions are met

## 📊 Trading Logic

### Buy Conditions
- Target wallet buys a token on Pump.fun
- Token meets minimum market cap threshold
- Wallet has sufficient SOL balance

### Sell Conditions
- Take profit reached (token price >= buy_price * TP)
- Stop loss reached (token price <= buy_price * SL)
- Target wallet sells the token

### Risk Management
- Market cap filtering prevents buying low-cap tokens
- Slippage protection ensures reasonable execution prices
- Auto-liquidation prevents holding losing positions

## 🔒 Security

- Private keys are loaded from environment variables
- No hardcoded credentials in the codebase
- MEV protection services prevent front-running
- Transaction simulation before execution

## 📈 Performance

- Sub-millisecond transaction execution
- Real-time monitoring with minimal latency
- Efficient memory usage with async/await patterns
- Automatic retry mechanisms for failed transactions

## 🐛 Troubleshooting

### Common Issues

1. **Connection Errors**: Check RPC endpoint and network connectivity
2. **Transaction Failures**: Verify wallet balance and slippage settings
3. **Monitoring Issues**: Ensure Yellowstone gRPC credentials are valid

### Debug Commands

```bash
# Test Yellowstone gRPC connection
cargo run ping_test

# Check wallet balance
solana balance

# View transaction logs
pm2 logs
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## ⚠️ Disclaimer

This software is for educational and research purposes. Trading cryptocurrencies involves significant risk. Use at your own risk and never invest more than you can afford to lose.

## 📞 Support

For support and questions:
- **Telegram**: @greenfox
- **GitHub Issues**: [Create an issue](https://github.com/moonbot777/Copy-trading-bot-rust/issues)

---

**Note**: This bot is designed for Pump.fun DEX specifically. Make sure you understand the risks involved in copy trading and always test with small amounts first.
