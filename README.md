# Dynamic NFT Metadata Generator for Solana


A powerful serverless API for generating dynamic NFT metadata and images on the Solana blockchain. This project provides a scalable solution for creating and managing NFT metadata with real-time updates and customization options.


## contacts
- [telegram](https://t.me/caterpillardev)
- [twitter](https://x.com/caterpillardev)



## Features

- **Dynamic Metadata Generation**: Generate and update NFT metadata on-the-fly
- **Image Generation**: Create custom NFT images with text and URI support
- **Event Ticket Support**: Specialized metadata generation for event tickets
- **Caching System**: Optimized performance with API Gateway caching
- **Multi-Cluster Support**: Compatible with different Solana clusters
- **TypeScript Support**: Built with type safety and modern development practices

##  API Endpoints

### Image Generation

- `GET /img/{mintId}`: Generate NFT image with custom parameters
- `GET /img`: Base image generation endpoint

### Metadata Generation

- `GET /metadata/{mintId}`: Generate NFT metadata with custom parameters
- `GET /metadata`: Base metadata generation endpoint

## Configuration

The project uses Serverless Framework for deployment and configuration. Key configuration files:

- `serverless.yml`: Main configuration file
- `tsconfig.json`: TypeScript configuration
- `.eslintrc.js`: ESLint configuration

## Project Structure

```
api/
├── img-generator/     # Image generation logic
├── metadata-generator/# Metadata generation logic
├── common/           # Shared utilities
├── idls/            # Interface definitions
└── serverless.yml   # Serverless configuration
```

## Security

- API Gateway caching with configurable TTL
- Environment variable management
- CORS support
- Binary media type handling
