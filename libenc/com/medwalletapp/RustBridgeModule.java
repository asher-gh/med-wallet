package com.medwalletapp;

class RustBridgeModule {
  private static native byte[] encryptByteArray(byte[] input);
  private static native byte[] decryptByteArray(byte[] input);

  static { System.loadLibrary("fhe_enc"); }

  public static void main(String[] args) {

    String input = "Hello World!";
    System.out.println("Plain text: " + input);
    byte[] cypher = encryptByteArray(input.getBytes());
    System.out.println("cypher: " + cypher);

    byte[] plainText = decryptByteArray(cypher);
    System.out.println("decrypted: " + plainText);
  }
}
