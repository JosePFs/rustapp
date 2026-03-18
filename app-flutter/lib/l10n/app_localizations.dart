import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
import 'app_localizations_es.dart';
import 'app_localizations_gl.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations? of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations);
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('es'),
    Locale('gl'),
  ];

  /// No description provided for @appTitle.
  ///
  /// In en, this message translates to:
  /// **'Eixe'**
  String get appTitle;

  /// No description provided for @commonFollowDevice.
  ///
  /// In en, this message translates to:
  /// **'Follow device'**
  String get commonFollowDevice;

  /// No description provided for @commonLanguage.
  ///
  /// In en, this message translates to:
  /// **'Language'**
  String get commonLanguage;

  /// No description provided for @commonEnglish.
  ///
  /// In en, this message translates to:
  /// **'English'**
  String get commonEnglish;

  /// No description provided for @commonSpanish.
  ///
  /// In en, this message translates to:
  /// **'Spanish'**
  String get commonSpanish;

  /// No description provided for @commonGalician.
  ///
  /// In en, this message translates to:
  /// **'Galician'**
  String get commonGalician;

  /// No description provided for @authSignIn.
  ///
  /// In en, this message translates to:
  /// **'Sign in'**
  String get authSignIn;

  /// No description provided for @authEmailLabel.
  ///
  /// In en, this message translates to:
  /// **'Email'**
  String get authEmailLabel;

  /// No description provided for @authPasswordLabel.
  ///
  /// In en, this message translates to:
  /// **'Password'**
  String get authPasswordLabel;

  /// No description provided for @authEmailHint.
  ///
  /// In en, this message translates to:
  /// **'name@example.com'**
  String get authEmailHint;

  /// No description provided for @authPasswordHint.
  ///
  /// In en, this message translates to:
  /// **'Your password'**
  String get authPasswordHint;

  /// No description provided for @authLoginFailedTitle.
  ///
  /// In en, this message translates to:
  /// **'Could not sign in'**
  String get authLoginFailedTitle;

  /// No description provided for @authLoginFailedWrongCredentials.
  ///
  /// In en, this message translates to:
  /// **'Incorrect email or password.'**
  String get authLoginFailedWrongCredentials;

  /// No description provided for @authLoginFailedGeneric.
  ///
  /// In en, this message translates to:
  /// **'Please try again.'**
  String get authLoginFailedGeneric;

  /// No description provided for @statusStartingApp.
  ///
  /// In en, this message translates to:
  /// **'Starting app…'**
  String get statusStartingApp;

  /// No description provided for @statusInitializingBridge.
  ///
  /// In en, this message translates to:
  /// **'Initializing Rust bridge…'**
  String get statusInitializingBridge;

  /// No description provided for @statusBridgeReady.
  ///
  /// In en, this message translates to:
  /// **'Bridge ready. You can sign in now.'**
  String get statusBridgeReady;

  /// No description provided for @statusCallingLogin.
  ///
  /// In en, this message translates to:
  /// **'Calling Rust login…'**
  String get statusCallingLogin;

  /// No description provided for @statusSavingFeedback.
  ///
  /// In en, this message translates to:
  /// **'Saving feedback…'**
  String get statusSavingFeedback;

  /// No description provided for @statusFeedbackSaved.
  ///
  /// In en, this message translates to:
  /// **'Feedback saved.'**
  String get statusFeedbackSaved;

  /// No description provided for @statusUpdatingSessionState.
  ///
  /// In en, this message translates to:
  /// **'Updating session state…'**
  String get statusUpdatingSessionState;

  /// No description provided for @statusSessionMarkedCompleted.
  ///
  /// In en, this message translates to:
  /// **'Session marked as completed.'**
  String get statusSessionMarkedCompleted;

  /// No description provided for @statusSessionSavedAsCompleted.
  ///
  /// In en, this message translates to:
  /// **'Session saved as completed.'**
  String get statusSessionSavedAsCompleted;

  /// No description provided for @statusSessionMarkedNotCompleted.
  ///
  /// In en, this message translates to:
  /// **'Session marked as not completed.'**
  String get statusSessionMarkedNotCompleted;

  /// No description provided for @statusSignedOut.
  ///
  /// In en, this message translates to:
  /// **'Signed out. You can sign in again.'**
  String get statusSignedOut;

  /// No description provided for @bootstrapWelcomeBack.
  ///
  /// In en, this message translates to:
  /// **'Welcome back'**
  String get bootstrapWelcomeBack;

  /// No description provided for @bootstrapContinue.
  ///
  /// In en, this message translates to:
  /// **'Continue'**
  String get bootstrapContinue;

  /// No description provided for @bootstrapRetry.
  ///
  /// In en, this message translates to:
  /// **'Retry'**
  String get bootstrapRetry;

  /// No description provided for @bootstrapUnableToStartTitle.
  ///
  /// In en, this message translates to:
  /// **'Unable to start the app'**
  String get bootstrapUnableToStartTitle;

  /// No description provided for @errorMissingSupabaseConfig.
  ///
  /// In en, this message translates to:
  /// **'Missing Supabase configuration. Pass SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.'**
  String get errorMissingSupabaseConfig;

  /// No description provided for @errorBridgeInitFailed.
  ///
  /// In en, this message translates to:
  /// **'Bridge initialization failed: {error}'**
  String errorBridgeInitFailed(Object error);

  /// No description provided for @errorRustCallFailed.
  ///
  /// In en, this message translates to:
  /// **'Rust call failed: {error}'**
  String errorRustCallFailed(Object error);

  /// No description provided for @statusSignedInLoadedPrograms.
  ///
  /// In en, this message translates to:
  /// **'Signed in as {profileType}. Loaded {count} program(s).'**
  String statusSignedInLoadedPrograms(String profileType, int count);

  /// No description provided for @patientHomeSignOut.
  ///
  /// In en, this message translates to:
  /// **'Sign out'**
  String get patientHomeSignOut;

  /// No description provided for @patientHomeNoProgramsTitle.
  ///
  /// In en, this message translates to:
  /// **'No programs assigned'**
  String get patientHomeNoProgramsTitle;

  /// No description provided for @patientHomeNoProgramsSubtitle.
  ///
  /// In en, this message translates to:
  /// **'Your specialist has not assigned any programs yet.'**
  String get patientHomeNoProgramsSubtitle;

  /// No description provided for @patientHomeNoProgramsBody.
  ///
  /// In en, this message translates to:
  /// **'If you think this is an error, contact your specialist.'**
  String get patientHomeNoProgramsBody;

  /// No description provided for @programsYourProgramsTitle.
  ///
  /// In en, this message translates to:
  /// **'Your programs'**
  String get programsYourProgramsTitle;

  /// No description provided for @programsAssignedCount.
  ///
  /// In en, this message translates to:
  /// **'{count} assigned'**
  String programsAssignedCount(int count);

  /// No description provided for @programsNoDescription.
  ///
  /// In en, this message translates to:
  /// **'No description available.'**
  String get programsNoDescription;

  /// No description provided for @programsProgressLabel.
  ///
  /// In en, this message translates to:
  /// **'Progress'**
  String get programsProgressLabel;

  /// No description provided for @programsProgressPercent.
  ///
  /// In en, this message translates to:
  /// **'{percent}%'**
  String programsProgressPercent(int percent);

  /// No description provided for @programsEffortPainSummary.
  ///
  /// In en, this message translates to:
  /// **'Effort: {effort} / 10 · Pain: {pain} / 10'**
  String programsEffortPainSummary(String effort, String pain);

  /// No description provided for @programDetailSelectProgram.
  ///
  /// In en, this message translates to:
  /// **'Select a program to see its details.'**
  String get programDetailSelectProgram;

  /// No description provided for @programDetailSelectDay.
  ///
  /// In en, this message translates to:
  /// **'Select a day'**
  String get programDetailSelectDay;

  /// No description provided for @programDetailRestDayLabel.
  ///
  /// In en, this message translates to:
  /// **'Day {dayNumber} • Rest day'**
  String programDetailRestDayLabel(int dayNumber);

  /// No description provided for @programDetailDayLabel.
  ///
  /// In en, this message translates to:
  /// **'Day {dayNumber}'**
  String programDetailDayLabel(int dayNumber);

  /// No description provided for @programDetailCompletionDateLabel.
  ///
  /// In en, this message translates to:
  /// **'Completion date'**
  String get programDetailCompletionDateLabel;

  /// No description provided for @programDetailSave.
  ///
  /// In en, this message translates to:
  /// **'Save'**
  String get programDetailSave;

  /// No description provided for @programDetailSaveAsCompleted.
  ///
  /// In en, this message translates to:
  /// **'Save as completed'**
  String get programDetailSaveAsCompleted;

  /// No description provided for @programDetailMarkAsNotCompleted.
  ///
  /// In en, this message translates to:
  /// **'Mark as not completed'**
  String get programDetailMarkAsNotCompleted;

  /// No description provided for @exerciseSetsReps.
  ///
  /// In en, this message translates to:
  /// **'{sets} sets · {reps} reps'**
  String exerciseSetsReps(int sets, int reps);

  /// No description provided for @exerciseEffortLabel.
  ///
  /// In en, this message translates to:
  /// **'Effort'**
  String get exerciseEffortLabel;

  /// No description provided for @exercisePainLabel.
  ///
  /// In en, this message translates to:
  /// **'Pain'**
  String get exercisePainLabel;

  /// No description provided for @exerciseCommentLabel.
  ///
  /// In en, this message translates to:
  /// **'Comment (optional)'**
  String get exerciseCommentLabel;

  /// No description provided for @exerciseCommentHint.
  ///
  /// In en, this message translates to:
  /// **'Add notes about pain, difficulty, or anything else…'**
  String get exerciseCommentHint;

  /// No description provided for @exerciseVideoPlaceholder.
  ///
  /// In en, this message translates to:
  /// **'Exercise video'**
  String get exerciseVideoPlaceholder;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) =>
      <String>['en', 'es', 'gl'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en':
      return AppLocalizationsEn();
    case 'es':
      return AppLocalizationsEs();
    case 'gl':
      return AppLocalizationsGl();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
