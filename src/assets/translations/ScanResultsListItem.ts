// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

export const ScanResultsListItem = {
   en: {
      itemOne: '1 item',
      itemsCount: '{{count}} items',
      lastModified: 'Edited',
      selectItem: 'Select {{name}}',
      fdaRequiredTooltip:
         'This folder requires Full Disk Access to be moved to Trash. You can explore it and move its contents to Trash.',
      protectedTooltip:
         'This system folder is required for macOS to work correctly and cannot be moved to Trash. You can explore it and move its contents to Trash.',
   },
   it: {
      itemOne: '1 elemento',
      itemsCount: '{{count}} elementi',
      lastModified: 'Modif.',
      selectItem: 'Seleziona {{name}}',
      fdaRequiredTooltip:
         'La cartella richiede Accesso Completo al Disco per essere spostata nel Cestino. Puoi esplorarla e spostare i suoi oggetti nel Cestino.',
      protectedTooltip:
         'La cartella di sistema è necessaria per il corretto funzionamento di macOS e non può essere spostata nel Cestino. Puoi esplorarla e spostare i suoi oggetti nel Cestino.',
   },
   es: {
      itemOne: '1 elemento',
      itemsCount: '{{count}} elementos',
      lastModified: 'Editado',
      selectItem: 'Seleccionar {{name}}',
      fdaRequiredTooltip:
         'Esta carpeta requiere Acceso Completo al Disco para ser movida a la Papelera. Puedes explorarla y mover su contenido a la Papelera.',
      protectedTooltip:
         'Esta carpeta del sistema es necesaria para el correcto funcionamiento de macOS y no se puede mover a la Papelera. Puedes explorarla y mover su contenido a la Papelera.',
   },
   zh: {
      itemOne: '1 项',
      itemsCount: '{{count}} 项',
      lastModified: '已编辑',
      selectItem: '选择 {{name}}',
      fdaRequiredTooltip:
         '将此文件夹移至废纸篓需要完全磁盘访问权限。您可以浏览并将其内容移至废纸篓。',
      protectedTooltip:
         '此系统文件夹是macOS正常运行所必需的，无法移至废纸篓。您可以浏览并将其内容移至废纸篓。',
   },
   ja: {
      itemOne: '1 項目',
      itemsCount: '{{count}} 項目',
      lastModified: '編集済',
      selectItem: '{{name}} を選択',
      fdaRequiredTooltip:
         'このフォルダーをゴミ箱に移動するには、フルディスクアクセスが必要です。探索してその内容をゴミ箱に移動できます。',
      protectedTooltip:
         'このシステムフォルダーはmacOSが正常に機能するために必要であり、ゴミ箱に移動できません。探索してその内容をゴミ箱に移動できます。',
   },
   ar: {
      itemOne: 'عنصر واحد',
      itemsCount: '{{count}} عناصر',
      lastModified: 'عُدّل',
      selectItem: 'اختيار {{name}}',
      fdaRequiredTooltip:
         'يتطلب هذا المجلد الوصول الكامل إلى القرص ليتم نقله إلى سلة المهملات. يمكنك استكشافه ونقل محتوياته إلى سلة المهملات.',
      protectedTooltip:
         'هذا المجلد النظام ضروري ليعمل macOS بشكل صحيح ولا يمكن نقله إلى سلة المهملات. يمكنك استكشافه ونقل محتوياته إلى سلة المهملات.',
   },
   ru: {
      itemOne: '1 элемент',
      itemsCount: '{{count}} элементов',
      lastModified: 'Изменено',
      selectItem: 'Выбрать {{name}}',
      fdaRequiredTooltip:
         'Для перемещения этой папки в корзину требуется полный доступ к диску. Вы можете исследовать ее и переместить ее содержимое в корзину.',
      protectedTooltip:
         'Эта системная папка необходима для правильной работы macOS и не может быть перемещена в корзину. Вы можете исследовать ее и переместить ее содержимое в корзину.',
   },
   fr: {
      itemOne: '1 élément',
      itemsCount: '{{count}} éléments',
      lastModified: 'Modifié',
      selectItem: 'Sélectionner {{name}}',
      fdaRequiredTooltip:
         'Ce dossier nécessite un accès complet au disque pour être déplacé dans la corbeille. Vous pouvez l\u0027explorer et déplacer son contenu dans la corbeille.',
      protectedTooltip:
         'Ce dossier système est nécessaire au bon fonctionnement de macOS et ne peut pas être déplacé dans la corbeille. Vous pouvez l\u0027explorer et déplacer son contenu dans la corbeille.',
   },
   pt: {
      itemOne: '1 item',
      itemsCount: '{{count}} itens',
      lastModified: 'Editado',
      selectItem: 'Selecionar {{name}}',
      fdaRequiredTooltip:
         'Esta pasta requer Acesso Total ao Disco para ser movida para o Lixo. Você pode explorá-la e mover seu conteúdo para o Lixo.',
      protectedTooltip:
         'Esta pasta do sistema é necessária para o funcionamento correto do macOS e não pode ser movida para o Lixo. Você pode explorá-la e mover seu conteúdo para o Lixo.',
   },
   de: {
      itemOne: '1 Element',
      itemsCount: '{{count}} Elemente',
      lastModified: 'Bearb.',
      selectItem: '{{name}} auswählen',
      fdaRequiredTooltip:
         'Dieser Ordner erfordert vollen Festplattenzugriff, um in den Papierkorb verschoben zu werden. Sie können ihn erkunden und seinen Inhalt in den Papierkorb verschieben.',
      protectedTooltip:
         'Dieser Systemordner ist für die korrekte Funktion von macOS erforderlich und kann nicht in den Papierkorb verschoben werden. Sie können ihn erkunden und seinen Inhalt in den Papierkorb verschieben.',
   },
} as const
