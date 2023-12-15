package com.medwalletapp;
import java.nio.charset.StandardCharsets;

class RustBridgeModule {
  // Pointer to a aes object
  private static native long createAES();
  private static native byte[] encryptByteArray(long ptr, byte[] input);
  private static native byte[] decryptByteArray(long ptr, byte[] input);
  private static native void destroyAES(long aes_ptr);
  private static long aes_ptr;
  static {
    System.loadLibrary("fhe_enc");
    aes_ptr = createAES();
  }

  public static void main(String[] args) {

    String input = "Hello World!";
    System.out.println("Plain text: " + input);

    byte[] cypher = encryptByteArray(aes_ptr, input.getBytes());
    System.out.println("cypher: " + cypher);

    String decrypted =
        new String(decryptByteArray(aes_ptr, cypher), StandardCharsets.UTF_8);

    System.out.println("Decrypted: " + decrypted);

    destroyAES(aes_ptr);
  }
}
