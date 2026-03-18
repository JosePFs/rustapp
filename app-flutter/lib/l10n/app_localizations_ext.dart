import 'package:flutter/widgets.dart';

import 'app_localizations.dart';

extension AppLocalizationsBuildContext on BuildContext {
  AppLocalizations get l10n {
    final l10n = AppLocalizations.of(this);
    assert(l10n != null, 'AppLocalizations missing in widget tree.');
    return l10n!;
  }
}

