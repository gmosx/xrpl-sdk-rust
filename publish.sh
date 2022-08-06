#!/usr/bin/sh

# Publish all crates to the crates.io registry

pushd xrpl_types; cargo publish; popd; sleep 5;
pushd xrpl_binary_codec; cargo publish; popd; sleep 5;
pushd xrpl_api; cargo publish; popd; sleep 5;
pushd xrpl_address_codec; cargo publish; popd; sleep 5;
pushd xrpl_sdk_jsonrpc; cargo publish; popd; sleep 5;
pushd xrpl_sdk_ws; cargo publish; popd; sleep 5;
pushd xrpl_cli; cargo publish; popd;
