import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  webpack: (config, { isServer }) => {
    config.watchOptions = {
      ignored: [
        "!**/target/idl/**",
        "**/node_modules",
      ],
    };
    return config;
  },
};

export default nextConfig;
