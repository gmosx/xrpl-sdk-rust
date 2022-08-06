#!/usr/bin/sh

# Publish all crates to the crates.io registry

pushd xrpl_types; cargo publish; popd;
pushd xrpl_binary_codec; cargo publish; popd;
pushd xrpl_api; cargo publish; popd;
pushd xrpl_address_codec; cargo publish; popd;
pushd xrpl_sdk_jsonrpc; cargo publish; popd;
pushd xrpl_sdk_ws; cargo publish; popd;
pushd xrpl_cli; cargo publish; popd;
