(function () {
  'use strict';
  if (window.electron) return;
  var nextId = 1, pending = new Map(), listeners = new Map();
  var invokeChannels = new Set(['store','localeChanged','openDialog','createShortcut','openLogsFolder','openHostFolder','getAppVersion','getProtocol','handleProtocol','passedChecks','checkRequirements','findSteam','isSteamDirectory','findGmod','isGmodDirectory','getGmodPid','startGmod','cancelGmodLaunch','checkStApiHealth','stApiGet','isHubWindowVisible','workshopPreview','workshopSubscribe','designLoad','designSave','designPickAsset','designImport','designExport','designStyles','designSelectInitialStyle','updateCheck','updateInstall','updateGetMode','updateSetMode','updateCurrentVersion']);
  var eventChannels = new Set(['notify','hostIntegrityFailed','gmodLaunchState','gmodUpdatePid','hubUpdate','hubWindowVisible','stApiBaseUrlChanged','animatedBackgroundChanged']);
  function transport(packet) {
    if (window.ipc && typeof window.ipc.postMessage === 'function') { window.ipc.postMessage(JSON.stringify(packet)); return; }
    if (window.chrome && chrome.webview && typeof chrome.webview.postMessage === 'function') { chrome.webview.postMessage(packet); return; }
    var error = new Error('Нативный хост Panopticon Hub недоступен.');error.code = 'NATIVE_HOST_UNAVAILABLE';throw error;
  }
  function receive(packet) {
    if (typeof packet === 'string') { try { packet = JSON.parse(packet); } catch (_) { return; } }
    if (!packet || typeof packet !== 'object') return;
    if (packet.kind === 'result') {
      var operation = pending.get(packet.id);if (!operation) return;pending.delete(packet.id);
      if (packet.ok === true) { operation.resolve(packet.isUndefined === true ? undefined : packet.value); return; }
      var details = packet.error || {}, error = new Error(typeof details.message === 'string' ? details.message : 'Ошибка нативного хоста Panopticon Hub.');
      if (typeof details.code === 'string') error.code = details.code;
      operation.reject(error);return;
    }
    if (packet.kind !== 'event' || !eventChannels.has(packet.channel) || !Array.isArray(packet.args)) return;
    var args = packet.args.slice();
    if (Array.isArray(packet.undefinedArgs)) packet.undefinedArgs.forEach(function (index) { if (Number.isInteger(index) && index >= 0 && index < args.length) args[index] = undefined; });
    var callbacks = listeners.get(packet.channel);if (!callbacks) return;
    Array.from(callbacks).forEach(function (callback) { try { callback.apply(undefined, args); } catch (error) { console.error('Ошибка обработчика события Hub:', packet.channel, error); } });
  }
  function invoke(channel, args) {
    return new Promise(function (resolve, reject) {
      var id = nextId++;pending.set(id, {resolve:resolve,reject:reject});
      try { transport({kind:'invoke',id:id,channel:channel,args:args}); } catch (error) { pending.delete(id);reject(error); }
    });
  }
  async function firstRun() {
    await window.panopticonInitialStyleReady?.();
    await window.panopticonWorkshopReady?.();
    window.panopticonUpdatesReady?.();
  }
  var api = {
    invoke: async function (channel) {
      var args = Array.prototype.slice.call(arguments, 1);
      if (!invokeChannels.has(channel)) throw new Error('Неизвестная операция Hub: ' + String(channel));
      if (channel === 'startGmod' && window.panopticonWorkshopReady) {
        await firstRun();
        try { return await invoke(channel, args); }
        catch (error) { if (error.message !== 'WORKSHOP_PREVIEW_READY') throw error; }
        await window.panopticonWorkshopReady(true);
        return invoke(channel, args);
      }
      var result = await invoke(channel, args);
      if (channel === 'passedChecks' || (channel === 'store' && args[0] === 'set' && args[1] === 'legalConsent')) {
        setTimeout(() => { firstRun().catch(error => console.error('Ошибка первоначальной настройки Hub:', error)); }, 0);
      }
      return result;
    },
    on: function (channel, callback) {
      if (!eventChannels.has(channel) || typeof callback !== 'function') throw new TypeError('Неизвестное событие Hub или некорректный обработчик.');
      if (!listeners.has(channel)) listeners.set(channel, new Set());listeners.get(channel).add(callback);
    },
    removeListener: function (channel, callback) { var callbacks = listeners.get(channel);if (callbacks) { callbacks.delete(callback);if (!callbacks.size) listeners.delete(channel); } },
    send: function (channel) {
      if (channel !== 'window-control') throw new Error('Неизвестная команда окна Hub.');
      var args = Array.prototype.slice.call(arguments, 1);
      if (args.length !== 1 || ['minimize','close','drag'].indexOf(args[0]) === -1) throw new Error('Некорректная команда окна Hub.');
      transport({kind:'send',channel:channel,args:args});
    }
  };
  Object.defineProperty(window, 'electron', {value:Object.freeze(api),writable:false,configurable:false});
  Object.defineProperty(window, '__panopticonReceive', {value:receive,writable:false,configurable:false});
  if (window.chrome && chrome.webview && typeof chrome.webview.addEventListener === 'function') chrome.webview.addEventListener('message', function (event) { receive(event.data); });
  window.addEventListener('beforeunload', function () { pending.forEach(function (operation) { operation.reject(new Error('Окно Hub закрыто.')); });pending.clear();listeners.clear(); });
}());
