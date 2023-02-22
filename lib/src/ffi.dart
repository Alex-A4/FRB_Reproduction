import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';
import 'package:frb_test/bridge.dart';

NekotonBridge? _wrapper;

NekotonBridge createWrapper(ExternalLibrary lib) {
  _wrapper ??= createWrapperImpl(lib);
  return _wrapper!;
}

NekotonBridge createLib() => createWrapper(createLibraryImpl());
